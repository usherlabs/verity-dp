//! The 'remittance' submodule contains logic for the remittance canister

pub mod external_router;
pub mod state;
pub mod types;
pub mod utils;

use crate::{
    crypto::{
        self,
        config::{Config, Environment},
        ethereum::{recover_address_from_eth_signature, sign_message},
        vec_u8_to_string,
    },
    owner, random,
};
use candid::Principal;
use ic_cdk::caller;
use state::*;
use types::Subscriber;

const REMITTANCE_EVENT: &str = "REMITTANCE";

/// Initializes the remittance canister state
pub fn init(env_opt: Option<Environment>) {
    owner::init_owner();
    random::init_ic_rand();

    // Save the environment configuration
    if let Some(env) = env_opt {
        CONFIG.with(|s| {
            let mut state = s.borrow_mut();
            *state = Config::from(env);
        })
    }
}

/// Returns the owner of the contract
pub fn owner() -> String {
    owner::get_owner()
}

/// Returns the name of the contract
pub fn name() -> String {
    format!("remittance canister")
}

/// Subscribes to a DC canister using its canister_id
pub async fn subscribe_to_dc(canister_id: Principal) {
    let subscriber = Subscriber {
        topic: REMITTANCE_EVENT.to_string(),
    };
    let _call_result: Result<(), _> = ic_cdk::call(canister_id, "subscribe", (subscriber,)).await;
    // Update the list of subscribed canisters, avoiding duplicates
    DC_CANISTERS.with(|dc_canister| {
        let mut borrowed_canister = dc_canister.borrow_mut();
        if !borrowed_canister.contains(&canister_id) {
            borrowed_canister.push(canister_id)
        }
    });
}

/// Subscribes to a PCD canister using its canister_id
pub async fn subscribe_to_pdc(pdc_canister_id: Principal) {
    subscribe_to_dc(pdc_canister_id).await;
    IS_PDC_CANISTER.with(|is_pdc_canister| {
        is_pdc_canister.borrow_mut().insert(pdc_canister_id, true);
    });
}

/// Validates and updates user and canister balances
pub fn update_remittance(
    new_remittances: Vec<types::DataModel>,
    dc_canister: Principal,
) -> Result<(), String> {
    let is_pdc =
        IS_PDC_CANISTER.with(|is_pdc_canister| is_pdc_canister.borrow().contains_key(&caller()));

    // Validate input data, returning errors if any
    if let Err(text) = utils::validate_remittance_data(is_pdc, &new_remittances, dc_canister) {
        return Err(text);
    }

    // Process each remittance message
    for new_remittance in new_remittances {
        let _: Result<(), String> = match new_remittance.action.clone() {
            types::Action::Adjust => {
                utils::update_balance(&new_remittance, dc_canister);
                Ok(())
            }
            types::Action::Deposit => {
                utils::update_balance(&new_remittance, dc_canister);
                // Increment canister's token balance on deposit
                utils::update_canister_balance(
                    new_remittance.token,
                    new_remittance.chain,
                    dc_canister,
                    new_remittance.amount,
                );
                Ok(())
            }
            types::Action::Withdraw => {
                utils::confirm_withdrawal(
                    new_remittance.token.to_string(),
                    new_remittance.chain.to_string(),
                    new_remittance.account.to_string(),
                    new_remittance.amount.abs() as u64,
                    dc_canister,
                );
                // Deduct withdrawn amount from canister's pool
                utils::update_canister_balance(
                    new_remittance.token,
                    new_remittance.chain,
                    dc_canister,
                    -new_remittance.amount,
                );
                Ok(())
            }
            types::Action::CancelWithdraw => {
                utils::cancel_withdrawal(
                    new_remittance.token.to_string(),
                    new_remittance.chain.to_string(),
                    new_remittance.account.to_string(),
                    new_remittance.amount.abs() as u64,
                    dc_canister,
                );
                Ok(())
            }
        };
    }

    Ok(())
}

/// Creates a remittance request
pub async fn remit(
    token: String,
    chain: String,
    account: String,
    dc_canister: Principal,
    amount: u64,
    proof: String,
) -> Result<types::RemittanceReply, Box<dyn std::error::Error>> {
    // Verify the 'proof' is a valid signature of the amount
    let _derived_address = recover_address_from_eth_signature(proof, format!("{amount}"))?;

    // Ensure the signature matches the provided account
    assert!(
        _derived_address == account.to_lowercase(),
        "INVALID_SIGNATURE"
    );
    // Ensure the remitted amount is greater than zero
    assert!(amount > 0, "AMOUNT < 0");

    // Generate key values
    let chain: types::Chain = chain.try_into()?;
    let token: types::Wallet = token.try_into()?;
    let account: types::Wallet = account.try_into()?;

    let hash_key = (
        token.clone(),
        chain.clone(),
        account.clone(),
        dc_canister.clone(),
    );

    // Check for withheld balance for the specified amount
    let withheld_balance = utils::get_remitted_balance(
        token.clone(),
        chain.clone(),
        account.clone(),
        dc_canister.clone(),
        amount,
    );

    let response: types::RemittanceReply;
    // Return cached signature and nonce if amount exists in withheld map
    if withheld_balance.balance == amount {
        let message_hash = utils::hash_remittance_parameters(
            withheld_balance.nonce,
            amount,
            &account.to_string(),
            &chain.to_string(),
            &dc_canister.to_string(),
            &token.to_string(),
        );

        response = types::RemittanceReply {
            hash: vec_u8_to_string(&message_hash),
            signature: withheld_balance.signature.clone(),
            nonce: withheld_balance.nonce,
            amount,
        };
    } else {
        let nonce = random::get_random_number();
        let message_hash = utils::hash_remittance_parameters(
            nonce,
            amount,
            &account.to_string(),
            &chain.to_string(),
            &dc_canister.to_string(),
            &token.to_string(),
        );
        let balance = get_available_balance(
            token.to_string(),
            chain.to_string(),
            account.to_string(),
            dc_canister.clone(),
        )?
        .balance;

        // Ensure sufficient funds for withdrawal
        if amount > balance {
            panic!("REMIT_AMOUNT:{amount} > AVAILABLE_BALANCE:{balance}");
        }

        // Generate a signature for the parameters
        let config_store = CONFIG.with(|store| store.borrow().clone());
        let signature_reply = sign_message(&message_hash, &config_store).await?;
        let signature_string = format!("0x{}", signature_reply.signature_hex);

        // Deduct remitted amount from main balance
        REMITTANCE.with(|remittance| {
            if let Some(existing_data) = remittance.borrow_mut().get_mut(&hash_key) {
                existing_data.balance = existing_data.balance - amount;
            }
        });
        // Track individual remitted amounts per (token, chain, recipient) combination
        WITHHELD_AMOUNTS.with(|withheld_amount| {
            withheld_amount
                .borrow_mut()
                .entry(hash_key.clone())
                .or_insert(Vec::new())
                .push(amount);
        });
        // Update withheld balance and generate a new signature
        WITHHELD_REMITTANCE.with(|withheld| {
            let mut withheld_remittance_store = withheld.borrow_mut();
            withheld_remittance_store.insert(
                (
                    token.clone(),
                    chain.clone(),
                    account.clone(),
                    dc_canister.clone(),
                    amount,
                ),
                types::WithheldAccount {
                    balance: amount,
                    signature: signature_string.clone(),
                    nonce,
                },
            );
        });
        // Create response object
        response = types::RemittanceReply {
            hash: format!("0x{}", vec_u8_to_string(&message_hash)),
            signature: signature_string.clone(),
            nonce,
            amount,
        };
    }

    Ok(response)
}

/// Retrieves the available balance for an account associated with a canister
pub fn get_available_balance(
    token: String,
    chain: String,
    account: String,
    dc_canister: Principal,
) -> Result<types::Account, Box<dyn std::error::Error>> {
    let chain: types::Chain = chain.try_into()?;
    let token: types::Wallet = token.try_into()?;
    let account: types::Wallet = account.try_into()?;

    // Get available balance for the specified key
    let amount = utils::get_available_balance(token, chain, account, dc_canister);

    Ok(amount)
}

/// Retrieves the canister balance for a specific token and chain
pub fn get_canister_balance(
    token: String,
    chain: String,
    dc_canister: Principal,
) -> Result<types::Account, Box<dyn std::error::Error>> {
    let chain: types::Chain = chain.try_into().unwrap();
    let token: types::Wallet = token.try_into().unwrap();

    // Get canister balance for the specified key
    let amount = utils::get_canister_balance(token, chain, dc_canister);

    Ok(amount)
}

/// Retrieves the withheld balance for an account on a specified canister and chain
pub fn get_withheld_balance(
    token: String,
    chain: String,
    account: String,
    dc_canister: Principal,
) -> Result<types::Account, Box<dyn std::error::Error>> {
    let chain: types::Chain = chain.try_into()?;
    let token: types::Wallet = token.try_into()?;
    let account: types::Wallet = account.try_into()?;

    let existing_key = (token.clone(), chain.clone(), account.clone(), dc_canister);

    // sum up all the amounts in the withheld_amount value of this key
    let sum = WITHHELD_AMOUNTS.with(|withheld_amount| {
        let withheld_amount = withheld_amount.borrow();
        let values = withheld_amount.get(&existing_key);

        match values {
            Some(vec) => vec.iter().sum::<u64>(),
            None => 0,
        }
    });

    Ok(types::Account { balance: sum })
}

/// Get the reciept of a successfull withdrawal
pub fn get_reciept(
    dc_canister: Principal,
    nonce: u64,
) -> Result<types::RemittanceReciept, Box<dyn std::error::Error>> {
    let key = (dc_canister.clone(), nonce.clone());
    Ok(REMITTANCE_RECIEPTS.with(|remittance_reciepts| {
        remittance_reciepts
            .borrow()
            .get(&key)
            .expect("RECIEPT_NOT_FOUND")
            .clone()
    }))
}

/// Get the public key associated with this particular canister
pub async fn public_key() -> Result<crypto::ecdsa::PublicKeyReply, Box<dyn std::error::Error>> {
    let config = CONFIG.with(|c| c.borrow().clone());

    let request = crypto::ecdsa::ECDSAPublicKey {
        canister_id: None,
        derivation_path: vec![],
        key_id: config.key.to_key_id(),
    };

    let (res,): (crypto::ecdsa::ECDSAPublicKeyReply,) = ic_cdk::call(
        Principal::management_canister(),
        "ecdsa_public_key",
        (request,),
    )
    .await
    .map_err(|e| format!("ECDSA_PUBLIC_KEY_FAILED: {}\t,Error_code:{:?}", e.1, e.0))?;

    let address = crypto::ethereum::get_address_from_public_key(res.public_key.clone())?;

    Ok(crypto::ecdsa::PublicKeyReply {
        sec1_pk: hex::encode(res.public_key),
        etherum_pk: address,
    })
}

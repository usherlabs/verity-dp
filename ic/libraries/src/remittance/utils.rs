use super::{
    types::{Account, RemittanceReciept, WithheldAccount},
    DC_CANISTERS,
};
use crate::crypto::string_to_vec_u8;
use candid::Principal;
use easy_hasher::easy_hasher;
use eth_encode_packed::{
    ethabi::{ethereum_types::U256, Address},
    SolidityDataType,
};
use ic_cdk::{api::time, caller};

/// Ensures the caller is a whitelisted DC canister, otherwise panics.
/// This function checks if the caller's principal ID is in the list of allowed DC canisters.
pub fn only_whitelisted_dc_canister() {
    let caller_principal_id = caller();
    if !DC_CANISTERS.with(|publisher| publisher.borrow().contains(&caller_principal_id)) {
        panic!("NOT_ALLOWED");
    }
}

/// Hashes the parameters needed for a remittance transaction.
///
/// # Parameters
/// - `nonce`: A unique number to ensure the transaction is unique.
/// - `amount`: The amount to be remitted.
/// - `address`: The recipient's Ethereum address.
/// - `chain_id`: The ID of the blockchain network.
/// - `dc_canister_id`: The ID of the DC canister.
/// - `token_address`: The address of the token being transferred.
///
/// # Returns
/// A vector of bytes representing the hashed parameters.
pub fn hash_remittance_parameters(
    nonce: u64,
    amount: u64,
    address: &str,
    chain_id: &str,
    dc_canister_id: &str,
    token_address: &str,
) -> Vec<u8> {
    // convert the address to bytes format
    let address: [u8; 20] = string_to_vec_u8(address).try_into().unwrap();
    let token_address: [u8; 20] = string_to_vec_u8(token_address).try_into().unwrap();

    // pack the encoded bytes
    let input = vec![
        SolidityDataType::Number(U256::from(nonce)),
        SolidityDataType::Number(U256::from(amount)),
        SolidityDataType::Address(Address::from(address)),
        SolidityDataType::String(chain_id),
        SolidityDataType::String(dc_canister_id),
        SolidityDataType::Address(Address::from(token_address)),
    ];
    let (_bytes, __) = eth_encode_packed::abi::encode_packed(&input);

    easy_hasher::raw_keccak256(_bytes.clone()).to_vec()
}

/// Retrieves the remitted balance for a given account and amount.
///
/// # Parameters
/// - `token`: The wallet token.
/// - `chain`: The blockchain chain.
/// - `account`: The user's wallet account.
/// - `dc_canister`: The principal of the DC canister.
/// - `amount`: The amount to check for remittance.
///
/// # Returns
/// A `WithheldAccount` containing the remitted balance or zero if no remittance exists.
pub fn get_remitted_balance(
    token: super::types::Wallet,
    chain: super::types::Chain,
    account: super::types::Wallet,
    dc_canister: Principal,
    amount: u64,
) -> WithheldAccount {
    let withheld_amount = super::WITHHELD_REMITTANCE.with(|withheld| {
        let existing_key = (token, chain, account.clone(), dc_canister, amount);
        withheld
            .borrow()
            .get(&existing_key)
            .cloned()
            .unwrap_or_default()
    });

    withheld_amount
}

/// Retrieves the total available balance for a user.
///
/// # Parameters
/// - `token`: The wallet token.
/// - `chain`: The blockchain chain.
/// - `account`: The user's wallet account.
/// - `dc_canister`: The principal of the DC canister.
///
/// # Returns
/// An `Account` containing the available balance.
pub fn get_available_balance(
    token: super::types::Wallet,
    chain: super::types::Chain,
    account: super::types::Wallet,
    dc_canister: Principal,
) -> Account {
    let available_amount = super::REMITTANCE.with(|remittance| {
        let existing_key = (token, chain, account, dc_canister);
        remittance
            .borrow()
            .get(&existing_key)
            .cloned()
            .unwrap_or_default()
    });

    available_amount
}

/// Retrieves the total balance available to a specific canister.
///
/// # Parameters
/// - `token`: The wallet token.
/// - `chain`: The blockchain chain.
/// - `dc_canister`: The principal of the DC canister.
///
/// # Returns
/// An `Account` containing the canister's balance.
pub fn get_canister_balance(
    token: super::types::Wallet,
    chain: super::types::Chain,
    dc_canister: Principal,
) -> Account {
    let canister_balance = super::CANISTER_BALANCE.with(|cb| {
        let existing_key = (token, chain, dc_canister);
        cb.borrow().get(&existing_key).cloned().unwrap_or_default()
    });

    canister_balance
}

/// Updates the balance for a specific account in a DC canister within the remittance mapping.
///
/// # Parameters
/// - `new_remittance`: The new remittance data model.
/// - `dc_canister`: The principal of the DC canister.
pub fn update_balance(new_remittance: &super::types::DataModel, dc_canister: Principal) {
    super::REMITTANCE.with(|remittance| {
        let mut remittance_store = remittance.borrow_mut();

        let hash_key = (
            new_remittance.token.clone(),
            new_remittance.chain.clone(),
            new_remittance.account.clone(),
            dc_canister.clone(),
        );

        if let Some(existing_data) = remittance_store.get_mut(&hash_key) {
            existing_data.balance =
                ((existing_data.balance as i64) + (new_remittance.amount as i64)) as u64;
        } else {
            remittance_store.insert(
                hash_key,
                Account {
                    balance: new_remittance.amount as u64,
                },
            );
        }
    });
}

/// Updates the canister's balance for a specific token.
///
/// # Parameters
/// - `token`: The wallet token.
/// - `chain`: The blockchain chain.
/// - `dc_canister`: The principal of the DC canister.
/// - `amount`: The amount to update the balance by.
pub fn update_canister_balance(
    token: super::types::Wallet,
    chain: super::types::Chain,
    dc_canister: Principal,
    amount: i64,
) {
    super::CANISTER_BALANCE.with(|canister_balance| {
        let mut canister_balance_store = canister_balance.borrow_mut();

        let hash_key = (token.clone(), chain.clone(), dc_canister.clone());

        if let Some(existing_data) = canister_balance_store.get_mut(&hash_key) {
            existing_data.balance = ((existing_data.balance as i64) + (amount as i64)) as u64;
        } else {
            canister_balance_store.insert(
                hash_key,
                Account {
                    balance: amount as u64,
                },
            );
        }
    });
}

/// Confirms a successful withdrawal by updating the necessary balances.
///
/// # Parameters
/// - `token`: The token involved in the transaction.
/// - `chain`: The blockchain chain.
/// - `account`: The user's wallet account.
/// - `amount_withdrawn`: The amount withdrawn.
/// - `dc_canister`: The principal of the DC canister.
///
/// # Returns
/// A boolean indicating the success of the withdrawal confirmation.
pub fn confirm_withdrawal(
    token: String,
    chain: String,
    account: String,
    amount_withdrawn: u64,
    dc_canister: Principal,
) -> bool {
    let chain: super::types::Chain = chain.try_into().unwrap();
    let token: super::types::Wallet = token.try_into().unwrap();
    let account: super::types::Wallet = account.try_into().unwrap();

    let hash_key = (
        token.clone(),
        chain.clone(),
        account.clone(),
        dc_canister.clone(),
    );

    // go through the witheld amounts and remove this amount from it
    super::WITHHELD_AMOUNTS.with(|witheld_amounts| {
        let mut mut_witheld_amounts = witheld_amounts.borrow_mut();
        let unwithdrawn_amounts = mut_witheld_amounts
            .get(&hash_key)
            .expect("WITHDRAWAL_CONFIRMATION_ERROR:AMOUNT_NOT_WITHELD")
            .into_iter()
            .filter(|&amount_to_withdraw| *amount_to_withdraw != amount_withdrawn)
            .cloned()
            .collect();
        mut_witheld_amounts.insert(hash_key, unwithdrawn_amounts);
    });

    // go through the witheld balance store and remove this amount from it
    let withdrawn_details = super::WITHHELD_REMITTANCE.with(|withheld_remittance| {
        let key = (
            token.clone(),
            chain.clone(),
            account.clone(),
            dc_canister.clone(),
            amount_withdrawn,
        );
        let withdrawn_balance = withheld_remittance.borrow().get(&key).unwrap().clone();
        withheld_remittance.borrow_mut().remove(&key);

        withdrawn_balance
    });

    // create a reciept entry here for a succcessfull withdrawal
    super::REMITTANCE_RECIEPTS.with(|remittance_reciepts| {
        remittance_reciepts.borrow_mut().insert(
            (dc_canister, withdrawn_details.nonce),
            RemittanceReciept {
                token: token.to_string(),
                chain: chain.to_string(),
                amount: amount_withdrawn,
                account: account.to_string(),
                timestamp: time(),
            },
        );
    });
    return true;
}

/// Cancels a withdrawal request, returning the withheld amount to the available balance.
///
/// # Parameters
/// - `token`: The token involved in the transaction.
/// - `chain`: The blockchain chain.
/// - `account`: The user's wallet account.
/// - `amount_canceled`: The amount to cancel.
/// - `dc_canister`: The principal of the DC canister.
///
/// # Returns
/// A boolean indicating the success of the cancellation.
pub fn cancel_withdrawal(
    token: String,
    chain: String,
    account: String,
    amount_canceled: u64,
    dc_canister: Principal,
) -> bool {
    let chain: super::types::Chain = chain.try_into().unwrap();
    let token: super::types::Wallet = token.try_into().unwrap();
    let account: super::types::Wallet = account.try_into().unwrap();

    let hash_key = (
        token.clone(),
        chain.clone(),
        account.clone(),
        dc_canister.clone(),
    );

    // go through the witheld amounts and remove this amount from it
    super::WITHHELD_AMOUNTS.with(|witheld_amounts| {
        let mut mut_witheld_amounts = witheld_amounts.borrow_mut();
        let unwithdrawn_amounts = mut_witheld_amounts
            .get(&hash_key)
            .expect("CANCEL_WITHDRAW_ERROR:AMOUNT_NOT_WITHELD")
            .into_iter()
            .filter(|&amount_to_withdraw| *amount_to_withdraw != amount_canceled)
            .cloned()
            .collect();
        mut_witheld_amounts.insert(hash_key.clone(), unwithdrawn_amounts);
    });

    // go through the witheld balance store and remove this amount from it
    super::WITHHELD_REMITTANCE.with(|withheld_remittance| {
        withheld_remittance.borrow_mut().remove(&(
            token.clone(),
            chain.clone(),
            account.clone(),
            dc_canister.clone(),
            amount_canceled,
        ));
    });

    // add the withheld total back to the available balance
    super::REMITTANCE.with(|remittance| {
        if let Some(existing_data) = remittance.borrow_mut().get_mut(&hash_key) {
            existing_data.balance = existing_data.balance + amount_canceled;
        }
    });

    return true;
}

/// Validates remittance data based on whether the caller is a PDC or not.
///
/// # Parameters
/// - `is_pdc`: A boolean indicating if the caller is a PDC.
/// - `new_remittances`: A vector of new remittance data models.
/// - `dc_canister`: The principal of the DC canister.
///
/// # Returns
/// A result indicating success or an error message.
pub fn validate_remittance_data(
    is_pdc: bool,
    new_remittances: &Vec<super::types::DataModel>,
    dc_canister: Principal,
) -> Result<(), String> {
    match is_pdc {
        true => validate_pdc_remittance_data(new_remittances, dc_canister),
        false => validate_dc_remittance_data(new_remittances, dc_canister),
    }
}

/// Validates remittance data for processing by a PDC.
///
/// # Parameters
/// - `new_remittances`: A vector of new remittance data models.
/// - `dc_canister`: The principal of the DC canister.
///
/// # Returns
/// A result indicating success or an error message.
pub fn validate_pdc_remittance_data(
    new_remittances: &Vec<super::types::DataModel>,
    dc_canister: Principal,
) -> Result<(), String> {
    // validate that all adjust operations lead to a sum of zero
    let adjust_operations: Vec<super::types::DataModel> = new_remittances
        .into_iter()
        .filter(|&single_remittance| single_remittance.action == super::types::Action::Adjust)
        .cloned()
        .collect();
    // apply the same validation of dc canisters to the adjust operations of a pdc canister
    if let Err(err_message) = validate_dc_remittance_data(&adjust_operations, dc_canister) {
        return Err(err_message);
    }

    // validate that all operations that are not "adjust" operations are positive amounts
    // other than adjusts we currently have no use for negative amounts operations
    // this can be later changed
    let non_adjust_operations_gt_0: Vec<&super::types::DataModel> = new_remittances
        .into_iter()
        .filter(|single_remittance| {
            single_remittance.action != super::types::Action::Adjust && single_remittance.amount < 0
        })
        .collect();
    if non_adjust_operations_gt_0.len() > 0 {
        return Err("NON_ADJUST_AMOUNT_MUST_BE_GT_0".to_string());
    }

    Ok(())
}

/// Validates remittance data for processing by a DC canister.
///
/// # Parameters
/// - `new_remittances`: A vector of new remittance data models.
/// - `dc_canister`: The principal of the DC canister.
///
/// # Returns
/// A result indicating success or an error message.
pub fn validate_dc_remittance_data(
    new_remittances: &Vec<super::types::DataModel>,
    dc_canister: Principal,
) -> Result<(), String> {
    // validate that all operations are adjust and the resultant of amounts is zero
    let amount_delta = new_remittances
        .iter()
        .fold(0, |acc, account| acc + account.amount);

    if amount_delta != 0 {
        return Err("SUM_ADJUST_AMOUNTS != 0".to_string());
    }

    // validate it is only adjust action provided
    let is_action_valid = new_remittances
        .iter()
        .all(|item| item.action == super::types::Action::Adjust);

    if !is_action_valid {
        return Err("INVALID_ACTION_FOUND".to_string());
    }

    // check for all the negative deductions and confirm that the owners have at least that much balance
    let mut sufficient_balance_error: Result<(), String> = Ok(());
    new_remittances
        .iter()
        .filter(|&item| item.amount < 0)
        .for_each(|item| {
            let existing_balance = get_available_balance(
                item.token.clone(),
                item.chain.clone(),
                item.account.clone(),
                dc_canister.clone(),
            );

            if existing_balance.balance < (item.amount.abs() as u64) {
                sufficient_balance_error = Err("INSUFFICIENT_USER_BALANCE".to_string());
            }
        });
    if let Err(_) = sufficient_balance_error {
        return sufficient_balance_error;
    }
    // check for all positive additions that the canister has enough balance to cover it
    let mut insufficient_canister_balance: Result<(), String> = Ok(());
    new_remittances
        .iter()
        .filter(|&item| item.amount > 0)
        .for_each(|item| {
            let existing_balance =
                get_canister_balance(item.token.clone(), item.chain.clone(), dc_canister.clone());

            if existing_balance.balance < (item.amount as u64) {
                insufficient_canister_balance = Err("INSUFFICIENT_CANISTER_BALANCE".to_string());
            }
        });

    insufficient_canister_balance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recover_address_from_eth_signature() {
        let nonce: u64 = 10;
        let amount: u64 = 100;
        let address = "0x5c8e3a7c16fa5cdde9f74751d6b2395176f05c55".to_string();
        let chain_id = "ethereum:5";
        let dc_canister_id = "br5f7-7uaaa-aaaaa-qaaca-cai";
        let token_address = "0x5c8e3a7c16fa5cdde9f74751d6b2395176f05c55";
        let hashed_output = [
            8, 134, 176, 185, 121, 53, 193, 199, 185, 42, 238, 73, 122, 96, 223, 42, 230, 175, 125,
            59, 72, 6, 36, 6, 38, 59, 74, 94, 51, 57, 117, 88,
        ]
        .to_vec();

        let recovered_address = hash_remittance_parameters(
            nonce,
            amount,
            &address,
            chain_id,
            dc_canister_id,
            token_address,
        );
        assert_eq!(recovered_address, hashed_output);
    }
}

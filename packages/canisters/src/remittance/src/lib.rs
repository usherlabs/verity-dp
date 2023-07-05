use candid::Principal;
use ic_cdk::caller;
use ic_cdk_macros::*;
use lib;
use sha2::{Digest, Sha256};
use std::cell::RefCell;

mod ecdsa;
mod ethereum;
mod remittance;
mod utils;

const REMITTANCE_EVENT: &str = "REMITTANCE";
thread_local! {
    static REMITTANCE: RefCell<remittance::Store> = RefCell::default();
    static OWNER: RefCell<Option<Principal>> = RefCell::default();
    static PUBLISHERS: RefCell<Vec<Principal>> = RefCell::default();
}

// ------- Access control
fn only_publisher() {
    let caller_principal_id = caller();
    if !PUBLISHERS.with(|publisher| publisher.borrow().contains(&caller_principal_id)) {
        panic!("NOT_ALLOWED");
    }
}

fn only_owner() {
    let caller_principal_id = caller();
    if !OWNER.with(|owner| owner.borrow().expect("NO_OWNER") == caller_principal_id) {
        panic!("NOT_ALLOWED");
    }
}

#[init]
fn init() {
    let caller_principal_id = caller();
    OWNER.with(|token| {
        token.replace(Some(caller_principal_id));
    })
}

// upon upgrade of contracts, state is  lost
// so we need to reinitialize important variables here
#[post_upgrade]
fn upgrade() {
    init();
}

// get deployer of contract
#[query]
fn owner() -> String {
    OWNER.with(|owner| owner.borrow().clone().expect("NO_OWNER").to_string())
}

// @dev test function
#[query]
fn greet(name: String) -> String {
    format!("Hello data_collection canister, {}!", name)
}

// W.I.P this would be called to fe
#[query]
fn request() -> String {
    // make sure this function can only be called by a registered user
    format!("Signature_response")
}

// we call this method, with the id of the data_collection canister
// this then subscribes the remittance canister to "REMITTANCE" events from the data cannister
#[update]
async fn setup_subscribe(publisher_id: Principal) {
    only_owner();
    let subscriber = lib::Subscriber {
        topic: REMITTANCE_EVENT.to_string(),
    };
    let _call_result: Result<(), _> = ic_cdk::call(publisher_id, "subscribe", (subscriber,)).await;
    // update the list of all the publishers subscribed to while avoiding duplicates
    PUBLISHERS.with(|publisher| {
        let mut borrowed_publisher = publisher.borrow_mut();
        if !borrowed_publisher.contains(&publisher_id) {
            borrowed_publisher.push(publisher_id)
        }
    });
}

// test to get the last publisher
// test to get the number of publishers
// test to check if the passed in principal is present in the vector
// #[query]
// fn includes_publisher(publisher_id: Principal) -> bool {
//     PUBLISHERS.with(|publisher| publisher.borrow().contains(&publisher_id))
// }

// this is an external function which is going to be called by  the data collection canister
// when there is a new data
#[update]
fn update_remittance(new_remittances: Vec<lib::DataModel>) {
    only_publisher();
    for new_remittance in new_remittances {
        update_balance(new_remittance);
    }
}

// it essentially uses the mapping (ticker, chain, recipientaddress) => {DataModel}
// so if an entry exists for a particular combination of (ticker, chain, recipientaddress)
// then the price is updated, otherwise the entry is created
fn update_balance(new_remittance: lib::DataModel) {
    only_publisher();
    REMITTANCE.with(|remittance| {
        let mut remittance_store = remittance.borrow_mut();

        let hash_key = (
            new_remittance.ticker.clone(),
            new_remittance.chain.clone(),
            new_remittance.recipient_address.clone(),
        );

        if let Some(existing_data) = remittance_store.get_mut(&hash_key) {
            existing_data.balance =
                (existing_data.balance as i64 + new_remittance.amount as i64) as u64;
        } else {
            remittance_store.insert(
                hash_key,
                remittance::Account {
                    balance: new_remittance.amount as u64,
                },
            );
        }
    });
}

// this function is just used to test and confirm if the data is actually included in the hashmap(object/dictionary)
// and if it can be queried, it would eventually be taken out when we get to testing
#[update]
async fn get_remittance(
    ticker: String,
    chain_name: String,
    chain_id: String,
    recipient_address: String,
) -> String {
    // validate the address and the chain
    if recipient_address.len() != 42 {
        panic!("INVALID_ADDRESS")
    };
    let chain = lib::Chain::from_chain_details(&chain_name, &chain_id).expect("INVALID_CHAIN");
    // validate the address and the chain

    let account = REMITTANCE.with(|remittance| {
        let existing_key = (ticker, chain, recipient_address.clone());
        remittance
            .borrow()
            .get(&existing_key)
            .expect("REMITTANCE_NOT_FOUND ")
            .clone()
    });

    // TODO use a random number generator to fetch the nonce
    let nonce = 10;
    let amount = account.balance;

    let (bytes_hash, _) =
        remittance::produce_remittance_hash(nonce, amount, &recipient_address[..]);

    let message = ethereum::sign_message(bytes_hash)
        .await
        .expect("ERROR_SIGNING_MESSAGE");

    // TODO return not only string but also information about the nonce and amount
    message.signature_hex
}

#[query]
fn get_balance(
    ticker: String,
    chain_name: String,
    chain_id: String,
    recipient_address: String,
) -> remittance::Account {
    // validate the address and the chain
    if recipient_address.len() != 42 {
        panic!("INVALID_ADDRESS")
    };
    let chain = lib::Chain::from_chain_details(&chain_name, &chain_id).expect("INVALID_CHAIN");
    // validate the address and the chain

    let amount = REMITTANCE.with(|remittance| {
        let existing_key = (ticker, chain, recipient_address.clone());
        remittance
            .borrow()
            .get(&existing_key)
            .expect("REMITTANCE_NOT_FOUND ")
            .clone()
    });

    amount
}

async fn derive_pk() -> Vec<u8> {
    let request = ecdsa::ECDSAPublicKey {
        canister_id: None,
        derivation_path: vec![],
        // TODO set this as an environment variable
        key_id: ecdsa::EcdsaKeyIds::TestKeyLocalDevelopment.to_key_id(),
    };

    let (res,): (ecdsa::ECDSAPublicKeyReply,) = ic_cdk::call(
        Principal::management_canister(),
        "ecdsa_public_key",
        (request,),
    )
    .await
    .map_err(|e| format!("ECDSA_PUBLIC_KEY_FAILED {}", e.1))
    .unwrap();

    return res.public_key;
}

#[update]
async fn public_key() -> Result<ecdsa::PublicKeyReply, String> {
    let request = ecdsa::ECDSAPublicKey {
        canister_id: None,
        derivation_path: vec![],
        // TODO set this as an environment variable
        key_id: ecdsa::EcdsaKeyIds::TestKeyLocalDevelopment.to_key_id(),
    };

    let (res,): (ecdsa::ECDSAPublicKeyReply,) = ic_cdk::call(
        Principal::management_canister(),
        "ecdsa_public_key",
        (request,),
    )
    .await
    .map_err(|e| format!("ECDSA_PUBLIC_KEY_FAILED {}", e.1))?;

    let address =
        ethereum::get_address_from_public_key(res.public_key.clone()).expect("INVALID_PUBLIC_KEY");

    Ok(ecdsa::PublicKeyReply {
        public_key_hex: hex::encode(res.public_key),
        etherum_pk: address,
    })
}

#[update]
async fn sign(message: String) -> Result<ecdsa::SignatureReply, String> {
    // hash the message to be signed
    let message_hash = ethereum::hash_eth_message(message.into_bytes());

    // sign the message
    let public_key = derive_pk().await;
    let request = ecdsa::SignWithECDSA {
        message_hash: message_hash.clone(),
        derivation_path: vec![],
        key_id: ecdsa::EcdsaKeyIds::TestKeyLocalDevelopment.to_key_id(),
    };

    let (response,): (ecdsa::SignWithECDSAReply,) = ic_cdk::api::call::call_with_payment(
        Principal::management_canister(),
        "sign_with_ecdsa",
        (request,),
        25_000_000_000,
    )
    .await
    .map_err(|e| format!("SIGN_WITH_ECDSA_FAILED {}", e.1))?;

    let full_signature = ethereum::get_signature(&response.signature, &message_hash, &public_key);
    Ok(ecdsa::SignatureReply {
        signature_hex: utils::vec_u8_to_string(&full_signature),
    })
}

#[query]
async fn verify(
    signature_hex: String,
    message: String,
    public_key_hex: String,
) -> Result<ecdsa::SignatureVerificationReply, String> {
    let signature_bytes = hex::decode(&signature_hex).expect("FAILED_TO_HEXDECODE_SIGNATURE");
    let pubkey_bytes = hex::decode(&public_key_hex).expect("FAILED_TO_HEXDECODE_PUBLIC_KEY");
    let message_bytes = ethereum::hash_eth_message(message.into_bytes());

    use k256::ecdsa::signature::Verifier;
    let signature = k256::ecdsa::Signature::try_from(signature_bytes.as_slice())
        .expect("DESERIALIZE_SIGNATURE_FAILED");
    let is_signature_valid = k256::ecdsa::VerifyingKey::from_sec1_bytes(&pubkey_bytes)
        .expect("DESERIALIZE_SEC1_ENCODING_FAILED")
        .verify(&message_bytes, &signature)
        .is_ok();

    Ok(ecdsa::SignatureVerificationReply { is_signature_valid })
}

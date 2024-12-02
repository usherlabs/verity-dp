use std::cell::RefCell;

use candid::Principal;
use ic_cdk::storage;

use utils::{
    burn_tokens_from_caller, generate_burn_payload, generate_mint_payload,
    get_user_canister_balance, mint_tokens_to_caller,
};
use verity_dp_ic::{
    crypto::ethereum::recover_address_from_eth_signature,
    remittance::{external_router, types::RemittanceSubscriber, update_remittance},
};

pub mod constants;
pub mod utils;

pub const ZERO_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

// set the address of the token
thread_local! {
    static TOKEN_PRINCIPAL: RefCell<Option<Principal>> = RefCell::default();
}

#[ic_cdk::update]
pub async fn mint(account: String, signature: String, amount: u128) {
    // validate the signature, which is a signature of the amount to be minted
    let recovered = recover_address_from_eth_signature(signature, amount.to_string()).unwrap();
    if recovered.to_lowercase() != account.to_lowercase() {
        panic!(
            "SIGNATURE_VERIFICATION_FAILED:recovered key: {}; public key:{}",
            recovered, account
        );
    }
    // validate the signature, which is a signature of the amount to be minted

    let dc_canister: Principal = ic_cdk::id();
    let caller = ic_cdk::caller();

    let user_balance = get_user_canister_balance(account.clone()).await;
    if user_balance < amount {
        panic!("INSUFFICIENT_FUNDS")
    }

    // deduct from the 'amount' user's balance and add it to the canister's balance
    let remittance_payload = generate_mint_payload(account.clone(), amount).await;
    update_remittance(remittance_payload, dc_canister).unwrap();

    // mint them some ccmatic tokens
    mint_tokens_to_caller(amount, &caller).await;
}

#[ic_cdk::update]
pub async fn burn(account: String, signature: String, amount: u128) {
    // validate the signature, which is a signature of the amount to be minted
    let recovered = recover_address_from_eth_signature(signature, amount.to_string()).unwrap();
    if recovered.to_lowercase() != account.to_lowercase() {
        panic!(
            "SIGNATURE_VERIFICATION_FAILED:recovered key: {}; public key:{}",
            recovered, account
        );
    }
    // validate the signature, which is a signature of the amount to be minted

    let dc_canister: Principal = ic_cdk::id();
    let caller = ic_cdk::caller();

    // first try to burn
    burn_tokens_from_caller(amount, &caller).await.unwrap();

    // add the 'amount' to the user's balance and add it to the canister's balance
    let remittance_payload = generate_burn_payload(account.clone(), amount).await;
    update_remittance(remittance_payload, dc_canister).unwrap();

    // mint them some ccmatic tokens
    mint_tokens_to_caller(amount, &caller).await;
}

// @dev testing command
#[ic_cdk::query]
fn name() -> String {
    format!("bridge_data_collection canister")
}

#[ic_cdk::init]
async fn init() {
    verity_dp_ic::owner::init_owner();
}

#[ic_cdk::query]
fn owner() -> String {
    verity_dp_ic::owner::get_owner()
}

#[ic_cdk::update]
pub async fn set_remittance_canister(remittance_principal: Principal) {
    verity_dp_ic::owner::only_owner();

    external_router::set_remittance_canister(remittance_principal);
}

#[ic_cdk::query]
pub fn get_remittance_canister() -> RemittanceSubscriber {
    // confirm at least one remittance canister is subscribed to this pdc
    external_router::get_remittance_canister()
}

// this function is going to be called by the remittance canister
// so it can recieve "publish" events from this canister
#[ic_cdk::update]
fn subscribe() {
    // verify if this remittance canister has been whitelisted
    // set the subscribed value to true if its the same, otherwise panic
    external_router::subscribe();
}

#[ic_cdk::query]
fn is_subscribed(canister_principal: Principal) -> bool {
    external_router::is_subscribed(canister_principal)
}

#[ic_cdk::update]
fn set_token_principal(token_canister_principal: Principal) {
    utils::set_token_principal(token_canister_principal)
}

#[ic_cdk::query]
fn get_token_principal() -> Principal {
    utils::get_token_principal()
}

#[ic_cdk::update]
async fn get_user_balance(account_address: String) -> u128 {
    get_user_canister_balance(account_address).await
}

#[ic_cdk::update]
async fn get_canister_balance() -> u128 {
    get_user_canister_balance(ZERO_ADDRESS.to_string()).await
}

// we would use this method to publish data to the subscriber
// which would be the remittance model
// so when we have some new data, we would publish it to the remittance model
#[ic_cdk::update]
async fn manual_publish(json_data: String) {
    // create a dummy remittance object we can publish until we implement data collection
    // which would then generate the data instead of hardcoding it
    let _ = external_router::publish_dc_json_to_remittance(json_data);
}

// --------------------------- upgrade hooks ------------------------- //
#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    let cloned_store = external_router::REMITTANCE_CANISTER.with(|rc| rc.borrow().clone());
    let cloned_token_principal = TOKEN_PRINCIPAL.with(|rc| rc.borrow().clone());
    storage::stable_save((cloned_store, cloned_token_principal)).unwrap()
}
#[ic_cdk::post_upgrade]
async fn post_upgrade() {
    init().await;

    let (old_store, cloned_token_principal): (Option<RemittanceSubscriber>, Option<Principal>) =
        storage::stable_restore().unwrap();
    external_router::REMITTANCE_CANISTER.with(|store| *store.borrow_mut() = old_store);
    TOKEN_PRINCIPAL.with(|store| *store.borrow_mut() = cloned_token_principal);
}
// --------------------------- upgrade hooks ------------------------- //

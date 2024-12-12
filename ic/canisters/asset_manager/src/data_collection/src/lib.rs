use candid::Principal;
use ic_cdk::storage;
use verity_ic::{ owner, remittance::{self, external_router}};

// @dev testing command
#[ic_cdk::query]
fn name() -> String {
    format!("data_collection canister")
}

#[ic_cdk::init]
async fn init() {
    owner::init_owner()
}

#[ic_cdk::query]
fn owner() -> String {
    owner::get_owner()
}

#[ic_cdk::update]
pub async fn set_remittance_canister(remittance_principal: Principal) {
    owner::only_owner();
    external_router::set_remittance_canister(remittance_principal);
}

#[ic_cdk::query]
pub fn get_remittance_canister() -> remittance::types::RemittanceSubscriber {
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

// we would use this method to publish data to the subscriber
// which would be the remittance model
// so when we have some new data, we would publish it to the remittance model
#[ic_cdk::update]
async fn manual_publish(json_data: String) -> Result<(), String> {
    // create a dummy remittance object we can publish until we implement data collection
    // which would then generate the data instead of hardcoding it
    let response = external_router::publish_dc_json_to_remittance(json_data).await;

    response
}

// --------------------------- upgrade hooks ------------------------- //
#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    let cloned_store = external_router::REMITTANCE_CANISTER.with(|rc| rc.borrow().clone());
    storage::stable_save((cloned_store,)).unwrap()
}
#[ic_cdk::post_upgrade]
async fn post_upgrade() {
    init().await;

    let (old_store,): (Option<remittance::types::RemittanceSubscriber>,) = storage::stable_restore().unwrap();
    external_router::REMITTANCE_CANISTER.with(|store| *store.borrow_mut() = old_store);
}
// --------------------------- upgrade hooks ------------------------- //

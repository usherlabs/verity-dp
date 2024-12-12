use candid::Principal;
use ic_cdk::{caller, storage};
use ic_cdk::{init, post_upgrade, pre_upgrade, query, update};
use std::{cell::RefCell, collections::HashMap};
use verity_ic::{
    crypto::config::{Config, Environment},
    owner,
    remittance::{
        external_router::{self, permissions},
        types::{self, RemittanceSubscriber},
    },
};

thread_local! {
    static CONFIG: RefCell<Config> = RefCell::default();
    static WHITELISTED_PUBLISHERS: RefCell<HashMap<Principal, bool>> = RefCell::default();
}

// ----------------------------------- init hook ------------------------------------------ //
#[init]
async fn init(env_opt: Option<Environment>) {
    owner::init_owner();

    // save the environment this is running in
    if let Some(env) = env_opt {
        CONFIG.with(|s| {
            let mut state = s.borrow_mut();
            *state = Config::from(env);
        })
    }
}
// ----------------------------------- init hook ------------------------------------------ //

#[query]
fn name() -> String {
    format!("protocol_data_collection canister")
}

/// get deployer of contract
#[query]
fn owner() -> String {
    owner::get_owner()
}

/// set remittance canister
#[update]
pub async fn set_remittance_canister(remittance_principal: Principal) {
    owner::only_owner();
    external_router::set_remittance_canister(remittance_principal);
}

/// get remittance canister
#[query]
pub fn get_remittance_canister() -> RemittanceSubscriber {
    external_router::get_remittance_canister()
}

/// whitelist publisher
#[update]
pub fn add_publisher(principal: Principal) {
    owner::only_owner();
    permissions::add_publisher(principal);
}

/// remove publisher
#[update]
pub fn remove_publisher(principal: Principal) {
    owner::only_owner();
    permissions::remove_publisher(principal);
}

/// this function is going to be called by the remittance canister
/// so it can recieve "publish" events from this canister
#[update]
fn subscribe() {
    external_router::subscribe()
}

#[update]
async fn manual_publish(json_data: String) {
    owner::only_owner();

    let _ = external_router::publish_pdc_json_to_remittance(json_data).await;
}


#[query]
fn is_subscribed(canister_principal: Principal) -> bool {
    let whitelisted_remittance_canister = get_remittance_canister();

    return whitelisted_remittance_canister.canister_principal == canister_principal
        && whitelisted_remittance_canister.subscribed;
}

#[query]
fn get_caller() -> Principal {
    caller()
}

// --------------------------- upgrade hooks ------------------------- //
#[pre_upgrade]
fn pre_upgrade() {
    let cloned_store = external_router::REMITTANCE_CANISTER.with(|rc| rc.borrow().clone());
    let config_store = CONFIG.with(|store| store.borrow().clone());
    let whitelisted_store = WHITELISTED_PUBLISHERS.with(|store| store.borrow().clone());

    storage::stable_save((cloned_store, config_store, whitelisted_store)).unwrap()
}
#[post_upgrade]
async fn post_upgrade() {
    let (old_store, cloned_config, whitelisted_store): (
        Option<types::RemittanceSubscriber>,
        Config,
        HashMap<Principal, bool>,
    ) = storage::stable_restore().unwrap();

    external_router::REMITTANCE_CANISTER.with(|store| *store.borrow_mut() = old_store);
    CONFIG.with(|c| *c.borrow_mut() = cloned_config);
    WHITELISTED_PUBLISHERS.with(|c| *c.borrow_mut() = whitelisted_store);

    owner::init_owner();
}
// --------------------------- upgrade hooks ------------------------- //

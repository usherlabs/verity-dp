//!
//! This submodule contains logic used to keep track of the owner of a canister
//! and potentially use it for authenticated/guarded canister methods
//! 
use std::cell::RefCell;

use candid::Principal;
use ic_cdk::caller;

thread_local! {
    static OWNER: RefCell<Option<Principal>> = RefCell::default();
}

/// A guard to be called at the beginning of a canister method
/// it will throw an error if the caller of the canister is not the owner of the canister
pub fn only_owner() {
    let caller_principal_id = caller();
    if !OWNER.with(|owner| owner.borrow().expect("NO_OWNER") == caller_principal_id) {
        panic!("NOT_ALLOWED");
    }
}

/// This function is to be called in the init hook of a canister
/// it is responsible for the initialisation of the owner variable
/// by storing the deployer of the canister as the owner
pub fn init_owner() {
    let caller_principal_id = caller();
    OWNER.with(|token| {
        token.replace(Some(caller_principal_id));
    });
}

/// get the owner of the canister
pub fn get_owner() -> String {
    OWNER.with(|owner| owner.borrow().clone().expect("NO_OWNER").to_string())
}

//! This submodule manages the owner of a canister and provides
//! functionality for authenticated/guarded canister methods.

use std::cell::RefCell;
use candid::Principal;
use ic_cdk::caller;

thread_local! {
	// Stores the owner of the canister as an optional Principal.
	static OWNER: RefCell<Option<Principal>> = RefCell::default();
}

/// Ensures that the caller of a canister method is the owner.
/// Panics if the caller is not the owner.
pub fn only_owner() {
	let caller_principal_id = caller();
	if !OWNER.with(|owner| owner.borrow().expect("NO_OWNER") == caller_principal_id) {
		panic!("NOT_ALLOWED");
	}
}

/// Initializes the owner variable during the canister's init hook.
/// Sets the deployer of the canister as the owner.
pub fn init_owner() {
	let caller_principal_id = caller();
	OWNER.with(|token| {
		token.replace(Some(caller_principal_id));
	});
}

/// Retrieves the owner of the canister as a string.
/// Panics if the owner has not been set.
pub fn get_owner() -> String {
	OWNER.with(|owner| owner.borrow().clone().expect("NO_OWNER").to_string())
}

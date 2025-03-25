//! The 'whitelist' module provides CRUD operations for managing a whitelist.
//! The whitelist is a HashMap where each principal is mapped to a boolean value.
//! A principal is considered whitelisted if it maps to `true`.

use std::{cell::RefCell, collections::HashMap};

use candid::Principal;

thread_local! {
    /// A thread-local storage for the whitelist, mapping principals to their whitelisted status.
    pub static WHITE_LIST: RefCell<HashMap<Principal, bool>> = RefCell::default();
}

/// Adds a principal to the whitelist.
pub fn add_to_whitelist(principal: Principal) {
    WHITE_LIST.with(|rc| rc.borrow_mut().insert(principal, true));
}

/// Removes a principal from the whitelist.
pub fn remove_from_whitelist(principal: Principal) {
    WHITE_LIST.with(|rc| rc.borrow_mut().remove(&principal));
}

/// Checks if a principal is whitelisted.
/// Returns `true` if the principal is whitelisted, otherwise `false`.
pub fn is_whitelisted(principal: Principal) -> bool {
    WHITE_LIST.with(|rc| rc.borrow().clone().contains_key(&principal))
}

//! 
//! The 'whitelist' submodule contains CRUD actions and a state variable
//! the whitelist variable  is essentially a hashmap of principal and bool
//! any whitelisted principal would be mapped to a boolean value of true
//! 

use std::{cell::RefCell, collections::HashMap};

use candid::Principal;

thread_local! {
    pub static WHITE_LIST: RefCell<HashMap<Principal, bool>> = RefCell::default();
}

/// Add a principal to the whitelist
pub fn add_to_whitelist(principal: Principal) {
    WHITE_LIST.with(|rc| rc.borrow_mut().insert(principal, true));
}

/// Remove a principal from the whitelist
pub fn remove_from_whitelist(principal: Principal) {
    WHITE_LIST.with(|rc| rc.borrow_mut().remove(&principal));
}

/// Returns the value representing if the input principal is whitelisted
pub fn is_whitelisted(principal: Principal) -> bool {
    WHITE_LIST.with(|rc| rc.borrow().clone().contains_key(&principal))
}

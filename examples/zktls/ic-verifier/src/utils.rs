use crate::state::CONFIG;
use verity_ic::{
    crypto::config::{Config, Environment},
    owner,
};

/// Initialize the canister's environment
pub fn init_canister(env_opt: Option<Environment>) {
    owner::init_owner();
    ic_wasi_polyfill::init(&[0u8; 32], &[]);

    // save the environment this is running in
    // defaults to Development
    if let Some(env) = env_opt {
        CONFIG.with(|s| {
            let mut state = s.borrow_mut();
            *state = Config::from(env);
        })
    }
}

pub fn ensure_sufficient_cycles(demand: u64) -> Result<(), String> {
    let balance = ic_cdk::api::canister_balance128();
    if balance < demand as u128 {
        Err(format!(
            "Insufficient cycles: have {}, need at least {}",
            balance, demand
        ))
    } else {
        Ok(())
    }
}

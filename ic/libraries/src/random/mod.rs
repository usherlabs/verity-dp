use rand::Rng;
use std::{cell::RefCell, time::Duration};

use candid::Principal;
use getrandom::register_custom_getrandom;
use rand::{rngs::StdRng, RngCore, SeedableRng};

thread_local! {
    pub static RNG: RefCell<Option<StdRng>> = RefCell::new(None);
}

/// Call this method in the init hook of the canister in order to make it usable across the canister
/// There is a need to call this method in the post update hook as well
pub fn init_ic_rand() {
    ic_cdk_timers::set_timer(Duration::from_secs(0), || ic_cdk::spawn(set_rand()));
    register_custom_getrandom!(custom_getrandom);
}

/// Call this method in order to get a random number for use in the canister which the `init_ic_rand` method has been called
pub fn get_random_number() -> u64 {
    RNG.with(|rng| rng.borrow_mut().as_mut().unwrap().gen())
}

async fn set_rand() {
    let (seed,) = ic_cdk::call(Principal::management_canister(), "raw_rand", ())
        .await
        .unwrap();
    RNG.with(|rng| {
        *rng.borrow_mut() = Some(StdRng::from_seed(seed));
        ic_cdk::println!("rng: {:?}", *rng.borrow());
    });
}

fn custom_getrandom(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    RNG.with(|rng| rng.borrow_mut().as_mut().unwrap().fill_bytes(buf));
    Ok(())
}

//! The 'random' submodule provides functionality for initializing and retrieving random numbers for use on the Internet Computer (IC).

// Import necessary modules and traits
use rand::Rng;
use std::{ cell::RefCell, time::Duration };

use candid::Principal;
use getrandom::register_custom_getrandom;
use rand::{ rngs::StdRng, RngCore, SeedableRng };

// Thread-local storage for the random number generator
thread_local! {
	pub static RNG: RefCell<Option<StdRng>> = RefCell::new(None);
}

/// Initializes the random number generator for the canister.
/// This function should be called in the canister's init hook and post-update hook.
pub fn init_ic_rand() {
	ic_cdk_timers::set_timer(Duration::from_secs(0), || ic_cdk::spawn(set_rand()));
	register_custom_getrandom!(custom_getrandom);
}

/// Retrieves a random number for use in the canister.
/// Assumes `init_ic_rand` has been called previously.
pub fn get_random_number() -> u64 {
	RNG.with(|rng| rng.borrow_mut().as_mut().unwrap().gen())
}

// Asynchronously sets the random number generator with a seed obtained from the management canister.
async fn set_rand() {
	let (seed,) = ic_cdk::call(Principal::management_canister(), "raw_rand", ()).await.unwrap();
	RNG.with(|rng| {
		*rng.borrow_mut() = Some(StdRng::from_seed(seed));
		ic_cdk::println!("rng: {:?}", *rng.borrow());
	});
}

// Custom implementation of the getrandom function to fill a buffer with random bytes.
fn custom_getrandom(buf: &mut [u8]) -> Result<(), getrandom::Error> {
	RNG.with(|rng| rng.borrow_mut().as_mut().unwrap().fill_bytes(buf));
	Ok(())
}

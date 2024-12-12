use crate::state::CONFIG;
use rs_merkle::{ algorithms::Sha256, Hasher };
use serde_json::Value;
use verity_ic::{ crypto::config::{ Config, Environment }, owner };

/// Initialise the canister's environment
pub fn init_canister(env_opt: Option<Environment>) {
	owner::init_owner();
	ic_wasi_polyfill::init(&[0u8; 32], &[]);

	// save the environment this is running in
	// defaults to staging
	if let Some(env) = env_opt {
		CONFIG.with(|s| {
			let mut state = s.borrow_mut();
			*state = Config::from(env);
		})
	}
}

/// Checks the proof json if it contains all the keys present in the vector of strings
/// returns an error if the provided json does not contain all of the provided keys
pub fn validate_json_proof(proof_json: &Value, json_keys: &Vec<&str>) -> bool {
	let condition = |key: &&str| proof_json.get(key).is_some();

	json_keys.iter().all(condition)
}

/// Perform a SHA 256 hash on the input string
/// Return a hex encoded value as the response
pub fn hash(input: &String) -> String {
	let input = input.as_bytes();
	// Convert the input string to bytes and hash it
	let hash = Sha256::hash(input);
	// Convert the resulting hash into a hexadecimal string and return it
	let hex_hash = hex::encode(hash);

	hex_hash
}

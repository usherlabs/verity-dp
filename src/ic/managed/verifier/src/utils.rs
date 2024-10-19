use anyhow::Result;
use easy_hasher::easy_hasher;
use eth_encode_packed::SolidityDataType;
use serde_json::Value;
use verity_dp_ic::{
    crypto::config::{Config, Environment},
    owner,
    remittance::state::CONFIG,
};

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
    // Convert the input string to bytes and hash it using Keccak-256
    let hex_hash = easy_hasher::keccak256(input).to_hex_string();
    // Convert the resulting hash into a hexadecimal string and return it
    hex_hash
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hash() {
        let input = "hello world".to_string();
        let expected_output =
            "47173285a8d7341e5e972fc677286384f802f8ef42a5ec5f03bbfa254cb01fad".to_string();
        let function_output = hash(&input);

        assert_eq!(function_output, expected_output)
    }
}

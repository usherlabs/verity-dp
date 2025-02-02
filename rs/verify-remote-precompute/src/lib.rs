pub mod config;
pub mod ic;

#[cfg(test)]
mod precompute_test {
	use config::Config;
	use ic::{ Verifier, DEFAULT_IC_GATEWAY_LOCAL };

	// Import everything from the outer scope
	use super::*;
	use std::fs;

	// Simple test
	#[tokio::test]
	async fn async_test_example() -> anyhow::Result<()> {
		// Read the file content into a string
		let proof1 = fs::read_to_string("./fixtures/proof.json")?;
		let proof2 = fs::read_to_string("./fixtures/session.json")?;
		let notary_pub_key = fs::read_to_string("./fixtures/notary.pub")?;

		// 1. Create a config file by specifying the params
		let config = Config::new(
			DEFAULT_IC_GATEWAY_LOCAL.to_string(),
			"./identity.pem".to_string(),
			"bkyz2-fmaaa-aaaaa-qaaaq-cai".to_string()
		);

		// 2. Create verifier from a config file
		let verifier = Verifier::from_config(&config).await.unwrap();

		// 3. verify a proof and get the response
		let response = verifier.verify_proof(vec![proof1, proof2], notary_pub_key).await;

		// get the public key of the canister for ecdsa signature verification
		let _ = verifier.get_public_key().await.unwrap();

		assert!(response.is_ok());

		Ok(())
	}
}

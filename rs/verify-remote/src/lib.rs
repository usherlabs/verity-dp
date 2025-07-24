pub mod config;
pub mod ic;

#[cfg(test)]
mod tests {
    use config::Config;
    use ic::{Verifier, DEFAULT_IC_GATEWAY_LOCAL};

    // Import everything from the outer scope
    use super::*;

    // Simple test
    #[tokio::test]
    async fn async_test_example() -> anyhow::Result<()> {
        // 1. Create a configuration by specifying the params
        let config = Config::new(
            DEFAULT_IC_GATEWAY_LOCAL.to_string(),
            verity_fixtures::ic::IDENTITY_PATH.to_string(),
            verity_fixtures::ic::VERIFIER.to_string(),
        );

        // 2. Create verifier from a config file
        let verifier = Verifier::from_config(&config).await.unwrap();

        // 3. verify a proof and get the response
        let response = verifier
            .verify_receipt(verity_fixtures::receipt::RECEIPT_1KB.to_vec())
            .await;

        // get the public key of the canister for ecdsa signature verification
        let _ = verifier.get_public_key().await.unwrap();

        assert!(
            response.is_ok(),
            "Expected Ok, got Error: {:?}",
            response.err().unwrap()
        );

        Ok(())
    }
}

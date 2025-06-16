pub mod config;
pub mod ic;

#[cfg(test)]
mod tests {
    use config::Config;
    use ic::{Verifier, DEFAULT_IC_GATEWAY_LOCAL};
    use k256::pkcs8::DecodePublicKey;

    // Import everything from the outer scope
    use super::*;

    // Simple test
    #[tokio::test]
    async fn async_test_example() -> anyhow::Result<()> {
        let proof = verity_fixtures::proof::PRESENTATION_32B.to_string();
        let notary_pub_key =
            k256::PublicKey::from_public_key_pem(verity_fixtures::notary::PUB_KEY)?;
        let notary_pub_key = notary_pub_key.to_sec1_bytes().into_vec();

        // 1. Create a configuration by specifying the params
        let config = Config::new(
            DEFAULT_IC_GATEWAY_LOCAL.to_string(),
            verity_fixtures::ic::IDENTITY_PATH.to_string(),
            "bkyz2-fmaaa-aaaaa-qaaaq-cai".to_string(),
        );

        // 2. Create verifier from a config file
        let verifier = Verifier::from_config(&config).await.unwrap();

        // 3. verify a proof and get the response
        let response = verifier.verify_proof(vec![proof], notary_pub_key).await;

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

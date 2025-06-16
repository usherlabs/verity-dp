pub mod config;
pub mod ic;

#[cfg(test)]
mod tests {
    use config::Config;
    use ic::{Verifier, DEFAULT_IC_GATEWAY_LOCAL};
    use k256::pkcs8::DecodePublicKey;

    // Import everything from the outer scope
    use super::*;
    use std::fs;

    // Simple test
    #[tokio::test]
    async fn async_test_example() -> anyhow::Result<()> {
        // Read the file content into a string
        let proof = fs::read_to_string("./fixtures/32b.presentation.json")?;
        let notary_pub_key = fs::read_to_string("./fixtures/notary.pub")?;
        let notary_pub_key = k256::PublicKey::from_public_key_pem(&notary_pub_key)?;
        let notary_pub_key = notary_pub_key.to_sec1_bytes().into_vec();

        // 1. Create a config file by specifying the params
        let config = Config::new(
            DEFAULT_IC_GATEWAY_LOCAL.to_string(),
            "./fixtures/identity.pem".to_string(),
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

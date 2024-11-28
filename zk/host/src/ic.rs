use std::error::Error;

use remote_verify::{config::Config, ic::Verifier};

/// This function uses the IC to verify a proof
/// provided the proofs, the public key of the notary
/// and a config specifying the details of the IC nevironment and verifier canister
pub async fn verify_proof(
    json_string_proofs: Vec<String>,
    notary_pub_key: String,
    config: Config,
) -> Result<remote_verify::ic::VerificationResponse, Box<dyn Error>> {
    // Create verifier from a config file
    let verifier = Verifier::from_config(&config).await.unwrap();

    // Verify a proof and get the response
    let response = verifier
        .verify_proof(json_string_proofs, notary_pub_key)
        .await;

    // return a response
    response
}

use std::error::Error;

use verity_verify_remote::{config::Config, ic::VerificationResponse, ic::Verifier};

/// Asynchronously verifies a proof using the Internet Computer (IC).
///
/// # Arguments
///
/// * `json_string_proofs` - A vector of JSON strings representing the proofs.
/// * `notary_pub_key` - The public key of the notary.
/// * `config` - Configuration details for the IC environment and verifier canister.
///
/// # Returns
///
/// * `Result<verity_verify_remote::ic::VerificationResponse, Box<dyn Error>>` -
///   A result containing the verification response or an error.
pub async fn verify_proof(
    json_string_proofs: Vec<String>,
    notary_pub_key: Vec<u8>,
    config: Config,
) -> Result<VerificationResponse, Box<dyn Error>> {
    // Initialize the verifier using the provided configuration
    let verifier = Verifier::from_config(&config).await.unwrap();

    // Perform the proof verification and obtain the response
    let response = verifier
        .verify_proof(json_string_proofs, notary_pub_key)
        .await;

    // Return the verification response
    response
}

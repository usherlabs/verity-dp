use std::error::Error;

use verity_remote_verify::{ config::Config, ic::Verifier, ic::VerificationResponse };

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
/// * `Result<verity_remote_verify::ic::VerificationResponse, Box<dyn Error>>` -
///   A result containing the verification response or an error.
pub async fn verify_proof(
	json_string_proofs: Vec<String>,
	notary_pub_key: String,
	config: Config
) -> Result<VerificationResponse, Box<dyn Error>> {
	// Initialize the verifier using the provided configuration
	let verifier = Verifier::from_config(&config).await.unwrap();

	// Perform the proof verification and obtain the response
	let response = verifier.verify_proof(json_string_proofs, notary_pub_key).await;

	// Return the verification response
	response
}

use candid::CandidType;
use serde::Deserialize;
use serde_json::Value;
use verity_ic::{
	crypto::ethereum::sign_message,
	remittance::state::CONFIG,
	verify::types::ProofResponse,
};
use verity_verify_tls::{ verify_proof, verify_session };

use crate::{ merkle::generate_merkle_tree, utils::{ hash, validate_json_proof } };

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct DirectVerificationResponse {
	pub results: Vec<ProofResponse>,
	pub root: String,
	pub signature: String,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ProofRequest {
	SessionProof(String),
	FullProof(String),
}

impl TryFrom<String> for ProofRequest {
	type Error = String;

	fn try_from(proof_string: String) -> Result<Self, Self::Error> {
		let full_proof_keys = vec!["session", "substrings"];
		let session_proof_keys = vec!["header", "signature", "session_info"];

		let proof_json: Value = serde_json::from_str(proof_string.as_str()).unwrap();

		// check the keys present in the proof to determine what kind of proof it is
		if validate_json_proof(&proof_json, &full_proof_keys) {
			return Ok(ProofRequest::FullProof(proof_string));
		}

		if validate_json_proof(&proof_json, &session_proof_keys) {
			return Ok(ProofRequest::SessionProof(proof_string));
		}

		Err("INVALID PROOF".to_string())
	}
}

impl ProofRequest {
	/// Depending on the kind of proof we are trying to verify
	/// Check and use the appropriate verifier on the input proof
	pub fn verify_request(&self, notary_pub_key: &String) -> Result<ProofResponse, String> {
		match self {
			// verify the session proof and return a hash of the input as a response
			ProofRequest::SessionProof(proof_string) => {
				let _ = verify_session(&proof_string, &notary_pub_key)?;
				let response = hash(&proof_string);
				Ok(ProofResponse::SessionProof(response))
			}
			// verify the full proof and return the request/response pair
			ProofRequest::FullProof(proof_string) => {
				let (res, req) = verify_proof(&proof_string, &notary_pub_key)?;
				let response = format!("{}\n\n{}", req, res);
				Ok(ProofResponse::FullProof(response))
			}
		}
	}
}

pub fn verify_proof_requests(
	proof_requests: Vec<String>,
	notary_pub_key: String
) -> Vec<ProofResponse> {
	// by default icp escapes special characters, so we need to unescape them
	let notary_pub_key = notary_pub_key.replace("\\n", "\n");

	// convert the string proofs to the actual type casted version of the proof
	let proof_requests: Vec<ProofRequest> = proof_requests
		.iter()
		.map(|proof_request| proof_request.clone().try_into().unwrap())
		.collect();

	// iterate through the proofs and try verifying them
	let proof_responses: Vec<ProofResponse> = proof_requests
		.iter()
		.map(|proof_request| { proof_request.clone().verify_request(&notary_pub_key).unwrap() })
		.collect();

	proof_responses
}

pub async fn verify_and_sign_proof_requests(
	proof_requests: Vec<String>,
	notary_pub_key: String
) -> Result<DirectVerificationResponse, String> {
	// iterate through the proofs and try verifying them
	let proof_responses: Vec<ProofResponse> = verify_proof_requests(proof_requests, notary_pub_key);

	// generate a merkle tree based on  the content of the proof responses as leaves
	let merkle_tree = generate_merkle_tree(&proof_responses);
	let merkle_root = merkle_tree.root().expect("NOT ENOUGH LEAVES");
	let merkle_root = hex::encode(merkle_root);

	// perform an ecdsa signature over this merkle root and return it
	// generate a signature for these parameters
	let config_store = CONFIG.with(|store| store.borrow().clone());
	let signature_reply = sign_message(&merkle_root.clone().into_bytes(), &config_store).await?;
	let signature = signature_reply.signature_hex;

	Ok(DirectVerificationResponse {
		results: proof_responses,
		root: merkle_root.clone(),
		signature,
	})
}

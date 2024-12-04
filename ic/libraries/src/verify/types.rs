use std::vec;

use anyhow::Result;
use candid::CandidType;
use serde::Deserialize;

/// The response from the managed verifier canister.
/// It is a `Result` type that contains either a `VerificationResponse` on success
/// or a `String` error message on failure.
pub type VerificationCanisterResponse = Result<VerificationResponse, String>;

/// Represents the response from the managed verifier canister.
/// It includes the proofs, a Merkle root derived from these proofs,
/// and the canister's ECDSA signature of the Merkle root.
#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct VerificationResponse {
	/// A vector of `ProofResponse` which indicates the source of each proof.
	pub results: Vec<ProofResponse>,
	/// The Merkle root encoded in hexadecimal format.
	pub root: String,
	/// The ECDSA signature of the Merkle root.
	pub signature: String,
}

/// Represents a proof verified by the managed verifier.
/// It can be either a `SessionProof` or a `FullProof`.
#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ProofResponse {
	SessionProof(String),
	FullProof(String),
}

// Implementations for the `ProofResponse` enum.
impl ProofResponse {
	/// Parses the HTTP response and extracts the JSON response body.
	/// This is applicable only for `FullProof`.
	pub fn get_http_response_body(&self) -> String {
		match self {
			ProofResponse::FullProof(text) => {
				let http_parts: Vec<&str> = text.split("\n\n").collect();
				let response = http_parts[1].to_string();

				let response_parts: Vec<&str> = response.split("\r\n\r\n").collect();
				let http_body = response_parts[1].to_string();

				http_body
			}
			ProofResponse::SessionProof(_) => panic!("Cannot extract HTTP response for session proof"),
		}
	}

	/// Retrieves the text content of a verified proof.
	pub fn get_content(&self) -> String {
		match self {
			// The result of a verified session proof is a hash, so no further processing is needed.
			ProofResponse::SessionProof(content) => content.clone(),
			// For a full proof, return the request/response pair.
			ProofResponse::FullProof(content) => content.clone(),
		}
	}
}

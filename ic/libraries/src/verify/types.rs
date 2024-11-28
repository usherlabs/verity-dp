use std::vec;

use anyhow::Result;
use candid::CandidType;
use serde::Deserialize;


/// The response from the managed verifier canister
pub type VerificationCanisterResponse = Result<VerificationResponse, String>;

/// The response from the managed verifier canister
/// containing the proofs and a merkle root built from the proofs
/// and the canister's ECDSA signature of the merkle root
#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct VerificationResponse {
    /// A vector of string proofs wrapped in a `ProofResponse` to know the source of the proof
    pub results: Vec<ProofResponse>,
    /// The hex encoded merkle root
    pub root: String,
    /// The ECDSA signature of the merkle root
    pub signature: String,
}

/// A Proof verified on the managed verifier could either be a SessionProof and FullProof
#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ProofResponse {
    SessionProof(String),
    FullProof(String),
}

// implementations for structs above
impl ProofResponse {
    /// Parse the HTTP response and extract the JSON response body
    pub fn get_http_response_body(&self) -> String {
        match self {
            ProofResponse::FullProof(text) => {
                let http_parts: Vec<&str> = text.split("\n\n").collect();
                let response = http_parts[1].to_string();

                let response_parts: Vec<&str> = response.split("\r\n\r\n").collect();
                let http_body = response_parts[1].to_string();

                http_body
            }
            ProofResponse::SessionProof(_) => panic!("cannot get http response for session proof"),
        }
    }

    /// Get the text content of a verified proof
    pub fn get_content(&self) -> String {
        match self {
            // the result of a verified session proof is a hash so no need to
            ProofResponse::SessionProof(content) => content.clone(),
            // verify the full proof and return the request/response pair
            ProofResponse::FullProof(content) => content.clone(),
        }
    }
}

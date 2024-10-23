use candid::CandidType;
use serde::Deserialize;

pub type CanisterVerificationResponse = Result<VerificationResponse, String>;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct VerificationResponse {
    pub results: Vec<ProofResponse>,
    pub root: String,
    pub signature: String,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ProofResponse {
    SessionProof(String),
    FullProof(String),
}

use std::error::Error;

use crate::config::Config;
use candid::{CandidType, Decode};
use ic_agent::{export::Principal, Agent};
use local_verify::{self, ecdsa::validate_ecdsa_signature, merkle::validate_merkle_tree};
use serde::Deserialize;
pub const DEFAULT_IC_GATEWAY_LOCAL: &str = "http://127.0.0.1:4943";
pub const DEFAULT_IC_GATEWAY_MAINNET: &str = "https://icp0.io";
pub const DEFAULT_IC_GATEWAY_MAINNET_TRAILING_SLASH: &str = "https://icp0.io/";

pub struct Verifier {
    pub agent: Agent,
    pub canister: Principal,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct PublicKeyReply {
    pub sec1_pk: String,
    pub etherum_pk: String,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct VerificationResponse {
    pub results: Vec<ProofResponse>,
    pub root: String,
    pub signature: String,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ProofResponse {
    SessionProof(String), //takes in reques response
    FullProof(String),    //takes in
}

impl ProofResponse {
    /// Helper function to fetch the content of a proof
    pub fn get_content(&self) -> String {
        match self {
            // the result of a verified session proof is a hash so no need to
            ProofResponse::SessionProof(content) => content.clone(),
            // verify the full proof and return the request/response pair
            ProofResponse::FullProof(content) => content.clone(),
        }
    }
}

type CanisterResponseType = Result<VerificationResponse, String>;

/// A proxy verifier to interfact with the managed verifier contract
impl Verifier {
    /// create a new verifier from a config struct
    pub async fn from_config(config: &Config) -> anyhow::Result<Self> {
        let agent = config.create_agent().await.unwrap();
        Ok(Self {
            agent,
            canister: config.canister_principal,
        })
    }

    /// verify a canisters' response
    /// by checking both the hash of the merkle root
    /// and if the ecdsa signature is valid
    async fn verify_canister_response(
        &self,
        verification_response: &VerificationResponse,
    ) -> Result<bool, Box<dyn Error>> {
        // extract all the parameters needed for verification
        let signature_hex = &verification_response.signature;
        let root_hash = &verification_response.root;
        let leaves: Vec<String> = verification_response
            .results
            .iter()
            .map(|proof_response| proof_response.get_content())
            .collect();
        let canister_public_key = self.get_public_key().await?;

        // verify the signature and the merkle tree root
        let is_signature_valid =
            validate_ecdsa_signature(signature_hex, &root_hash, &canister_public_key)?;
        let is_merkle_valid = validate_merkle_tree(&leaves, root_hash);

        // return this response
        let is_response_valid = is_signature_valid && is_merkle_valid;
        Ok(is_response_valid)
    }

    /// Get the public key of the specified canister
    pub async fn get_public_key(&self) -> Result<String, Box<dyn Error>> {
        let method_name = "public_key";

        // call the public key method on the specified canister
        let response = self
            .agent
            .update(&self.canister, method_name)
            .with_arg(candid::encode_args(())?)
            .call_and_wait()
            .await?;

        let public_key_response = Decode!(&response, PublicKeyReply)?;

        Ok(public_key_response.etherum_pk)
    }

    /// Verifies a proof on chain and validates the response locally
    pub async fn verify_proof(
        &self,
        string_proofs: Vec<String>,
        notary_pub_key: String,
    ) -> Result<VerificationResponse, Box<dyn Error>> {
        let verifier_method = "verify_proof_direct";

        // make the call to ic using the agent to verify the proof via the direct interface
        let response = self
            .agent
            .update(&self.canister, verifier_method)
            .with_arg(candid::encode_args((string_proofs, notary_pub_key))?)
            .call_and_wait()
            .await
            .unwrap();

        // parse the response into the right struct and return
        let verification_response = Decode!(&response, CanisterResponseType)??;

        // validate the signature and merkle tree
        let is_response_valid = self.verify_canister_response(&verification_response).await?;

        assert!(is_response_valid, "INVALID_CANISTER_RESPONSE");

        Ok(verification_response)
    }
}

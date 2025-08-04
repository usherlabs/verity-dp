use std::error::Error;

use crate::config::Config;
use candid::{CandidType, Decode};
use ic_agent::{export::Principal, Agent};
use serde::Deserialize;
// use verity_verify_local::{self, ecdsa::validate_ecdsa_signature, merkle::validate_merkle_tree};
pub const DEFAULT_IC_GATEWAY_LOCAL: &str = "http://127.0.0.1:4943";
pub const DEFAULT_IC_GATEWAY_MAINNET: &str = "https://icp0.io";
pub const DEFAULT_IC_GATEWAY_MAINNET_TRAILING_SLASH: &str = "https://icp0.io/";

pub struct Verifier {
    pub agent: Agent,
    pub canister: Principal,
}

/// Structure to hold public key reply data
#[derive(CandidType, Deserialize, Debug)]
pub struct PublicKeyReply {
    /// SEC1 formatted public key
    pub sec1_pk: String,
    /// Ethereum formatted public key
    pub etherum_pk: String,
}

/// Structure to hold receipt verification reply data
#[derive(CandidType, Deserialize, Debug)]
pub struct VerifyReceiptReply {
    /// Arbitrary data committed by zkVM program, serialized by bincode
    pub data: Vec<u8>,
    /// Signature in hexadecimal format
    pub signature: String,
}

/// A proxy verifier to interact with the managed verifier contract
impl Verifier {
    /// Creates a new verifier from a config struct
    pub async fn from_config(config: &Config) -> anyhow::Result<Self> {
        let agent = config.create_agent().await?;
        Ok(Self {
            agent,
            canister: config.canister_principal,
        })
    }

    /// Retrieves the public key of the specified canister
    pub async fn get_public_key(&self) -> Result<String, Box<dyn Error>> {
        let method_name = "public_key";

        // Calls the public key method on the specified canister
        let response = self
            .agent
            .update(&self.canister, method_name)
            .with_arg(candid::encode_args(())?)
            .call_and_wait()
            .await?;

        let public_key_response = Decode!(&response, PublicKeyReply)?;

        Ok(public_key_response.etherum_pk)
    }

    /// Verifies zkVM receipt, extracts its data and return it along with a signature
    pub async fn verify_receipt(
        &self,
        receipt: Vec<u8>,
    ) -> Result<VerifyReceiptReply, Box<dyn std::error::Error>> {
        let verifier_method = "verify_receipt";

        let response = self
            .agent
            .update(&self.canister, verifier_method)
            .with_arg(candid::encode_one(receipt)?)
            .call_and_wait()
            .await?;

        let result = Decode!(&response, Result<VerifyReceiptReply, String>)?;

        result.map_err(|e| e.into())
    }
}

#![allow(dead_code)]

use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::ecdsa::{EcdsaCurve, EcdsaKeyId};
use serde::{Deserialize, Serialize};

use super::config::Config;

/// Structure to hold public key reply data
#[derive(CandidType, Serialize, Debug)]
pub struct PublicKeyReply {
    /// SEC1 formatted public key
    pub sec1_pk: String,
    /// Ethereum formatted public key
    pub etherum_pk: String,
}

/// Structure to hold receipt verification reply data
#[derive(CandidType, Serialize, Debug)]
pub struct VerifyReceiptReply {
    /// Arbitrary data committed by zkVM program, serialized by bincode
    pub data: Vec<u8>,
    /// Signature in hexadecimal format
    pub signature: String,
}

// Structure to hold signature reply data
#[derive(CandidType, Serialize, Debug)]
pub struct SignatureReply {
    pub signature_hex: String, // Signature in hexadecimal format
}

// Structure to hold signature verification reply data
#[derive(CandidType, Serialize, Debug)]
pub struct SignatureVerificationReply {
    pub is_signature_valid: bool, // Boolean indicating if the signature is valid
}

// Type alias for CanisterId using Principal
type CanisterId = Principal;

// Structure to request an ECDSA public key
#[derive(CandidType, Serialize, Debug)]
pub struct ECDSAPublicKey {
    pub canister_id: Option<CanisterId>, // Optional canister ID
    pub derivation_path: Vec<Vec<u8>>,   // Derivation path for key generation
    pub key_id: EcdsaKeyId,              // Identifier for the ECDSA key
}

// Structure to hold the reply for an ECDSA public key request
#[derive(CandidType, Deserialize, Debug)]
pub struct ECDSAPublicKeyReply {
    pub public_key: Vec<u8>, // The derived public key
    pub chain_code: Vec<u8>, // Chain code associated with the key
}

// Structure to request signing with ECDSA
#[derive(CandidType, Serialize, Debug)]
pub struct SignWithECDSA {
    pub message_hash: Vec<u8>,         // Hash of the message to be signed
    pub derivation_path: Vec<Vec<u8>>, // Derivation path for key generation
    pub key_id: EcdsaKeyId,            // Identifier for the ECDSA key
}

// Structure to hold the reply for an ECDSA signing request
#[derive(CandidType, Deserialize, Debug)]
pub struct SignWithECDSAReply {
    pub signature: Vec<u8>, // The generated signature
}

// Enumeration of ECDSA key identifiers
#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum EcdsaKeyIds {
    #[allow(unused)]
    TestKeyLocalDevelopment, // Key for local development
    #[allow(unused)]
    TestKey1, // Test key 1
    #[allow(unused)]
    ProductionKey1, // Production key 1
}

// Implementation of methods for EcdsaKeyIds
impl EcdsaKeyIds {
    // Convert EcdsaKeyIds to EcdsaKeyId
    pub fn to_key_id(&self) -> EcdsaKeyId {
        EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1, // Use secp256k1 curve
            name: (match self {
                Self::TestKeyLocalDevelopment => "dfx_test_key",
                Self::TestKey1 => "test_key_1",
                Self::ProductionKey1 => "key_1",
            })
            .to_string(),
        }
    }
}

// Asynchronous function to derive a public key
pub async fn derive_pk(config: &Config) -> Vec<u8> {
    // Create a request for the ECDSA public key
    let request = ECDSAPublicKey {
        canister_id: None,              // No specific canister ID
        derivation_path: vec![],        // Empty derivation path
        key_id: config.key.to_key_id(), // Convert config key to EcdsaKeyId
    };

    // Call the management canister to get the ECDSA public key
    let (res,): (ECDSAPublicKeyReply,) = ic_cdk::call(
        Principal::management_canister(),
        "ecdsa_public_key",
        (request,),
    )
    .await
    .map_err(|e| format!("ECDSA_PUBLIC_KEY_FAILED: {}\t,Error_code:{:?}", e.1, e.0))
    .unwrap();

    // Return the derived public key
    res.public_key
}

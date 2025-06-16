use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};
use serde_json;
use verity_verify_local::{self, ecdsa::validate_ecdsa_signature, merkle::validate_merkle_tree};
use verity_verify_tls::verify_proof;

/// The input parameters for the zk_circuit
///
/// Contains the details needed for proof verification
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZkInputParam {
    /// Session header information
    pub tls_proof: String,
    /// Notary public key
    pub notary_pub_key: Vec<u8>,
    /// Precompute encodings
    pub encodings: Option<Vec<u8>>,
    /// Proof of substrings
    pub remote_verifier_proof: String,
    /// Remote verifier's ECDSA public key
    pub remote_verifier_public_key: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RemoteVerificationProof {
    pub results: Vec<String>,
    pub root: String,
    pub signature: String,
}

fn main() {
    // Read the input data for this application.
    let input_bytes: Vec<u8> = env::read();

    let params: String = String::from_utf8(input_bytes).unwrap();
    let params: ZkInputParam = serde_json::from_str(params.as_str()).unwrap();

    // Verify the Tls proof -- partially.
    let (recv, sent) =
        verify_proof(&params.tls_proof, &params.notary_pub_key, params.encodings).unwrap();

    // Verify the remote verifier's verification of the other part.
    let remote_verification_proof: RemoteVerificationProof =
        serde_json::from_str(params.remote_verifier_proof.as_str()).unwrap();

    // Verify the signature and the Merkle tree root
    let root_hash = &remote_verification_proof.root;
    let is_signature_valid = validate_ecdsa_signature(
        &remote_verification_proof.signature,
        root_hash,
        &params.remote_verifier_public_key,
    )
    .unwrap();
    let is_merkle_valid = validate_merkle_tree(&remote_verification_proof.results, root_hash);

    // Return the verification result
    assert!(is_signature_valid && is_merkle_valid);

    // write public output to the journal
    env::commit(&recv);
    env::commit(&sent);
}

use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};
use verity_ic::crypto::ethereum::sign_message;
use verity_verify_tls::PayloadBatch;

use crate::state::CONFIG;

/// Generates a Merkle tree from a vector of PayloadBatch objects.
/// Each Payload is hashed to create the leaves of the tree.
pub fn generate_merkle_tree(batches: &Vec<PayloadBatch>) -> MerkleTree<Sha256> {
    // Convert each Payload of the batches into a 32-byte hash to serve as a leaf in the Merkle tree.
    let leaves: Vec<[u8; 32]> = batches
        .iter()
        .flat_map(|batch| {
            batch.payloads.iter().map(|payload| {
                let payload_bytes = [payload.sent.as_bytes(), payload.received.as_bytes()].concat();
                Sha256::hash(&payload_bytes)
            })
        })
        .collect();

    // Construct the Merkle tree from the hashed leaves.
    let tree: MerkleTree<Sha256> = MerkleTree::<Sha256>::from_leaves(&leaves);
    return tree;
}

pub async fn sign_merkle_root(merkle_root: [u8; 32]) -> Result<String, Box<dyn std::error::Error>> {
    let merkle_root = hex::encode(merkle_root);

    // perform an ecdsa signature over this merkle root and return it
    // generate a signature for these parameters
    let config_store = CONFIG.with(|store| store.borrow().clone());
    let signature_reply = sign_message(&merkle_root.clone().into_bytes(), &config_store).await?;
    let signature = signature_reply.signature_hex;

    Ok(signature)
}

use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};
use verity_verify_tls::PayloadBatch;

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

/// Validate that the provided root hash is the same as the one derived from building a tree out of the provided leaves
pub fn validate_merkle_tree(batches: &Vec<PayloadBatch>, root_hash: &String) -> bool {
    let tree = generate_merkle_tree(&batches);

    let root = tree.root().unwrap();
    let derived_root_hash = hex::encode(root);
    let root_hash = root_hash.to_owned();

    // println!("derived_root_hash: {}", derived_root_hash);
    // println!("root_hash: {}", root_hash);

    return derived_root_hash == root_hash;
}

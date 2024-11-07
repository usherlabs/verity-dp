use crate::proof::ProofResponse;
use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};

pub fn generate_merkle_tree(leaves: &Vec<ProofResponse>) -> MerkleTree<Sha256> {
    // gather the leaves from the content of the verified proof
    // which is either the req/res pair or the has of the session proof
    let leaves: Vec<[u8; 32]> = leaves
        .iter()
        .map(|proof_response| {
            let proof_text_content = proof_response.get_content();
            let proof_byte_content = proof_text_content.as_bytes();

            Sha256::hash(proof_byte_content)
        })
        .collect();

    let tree: MerkleTree<Sha256> = MerkleTree::<Sha256>::from_leaves(&leaves);
    return tree;
}

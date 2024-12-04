use rs_merkle::{ algorithms::Sha256, Hasher, MerkleTree };
use verity_dp_ic::verify::types::ProofResponse;

/// Generates a Merkle tree from a vector of ProofResponse objects.
/// Each ProofResponse is hashed to create the leaves of the tree.
pub fn generate_merkle_tree(leaves: &Vec<ProofResponse>) -> MerkleTree<Sha256> {
	// Convert each ProofResponse into a 32-byte hash to serve as a leaf in the Merkle tree.
	let leaves: Vec<[u8; 32]> = leaves
		.iter()
		.map(|proof_response| {
			let proof_text_content = proof_response.get_content();
			let proof_byte_content = proof_text_content.as_bytes();

			Sha256::hash(proof_byte_content)
		})
		.collect();

	// Construct the Merkle tree from the hashed leaves.
	let tree: MerkleTree<Sha256> = MerkleTree::<Sha256>::from_leaves(&leaves);
	return tree;
}

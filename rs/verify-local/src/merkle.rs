// use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};

// /// Generate a merkle tree by providing the leaves as hashed strings
// pub fn generate_merkle_tree(leaves: &Vec<[u8; 32]>) -> MerkleTree<Sha256> {
//     let tree: MerkleTree<Sha256> = MerkleTree::<Sha256>::from_leaves(&leaves);
//     return tree;
// }

// /// Validate that the provided root hash is the same as the one derived from building a tree out of the provided leaves
// pub fn validate_merkle_tree(leaves: &Vec<String>, root_hash: &String) -> bool {
//     // gather the leaves from the content of the verified proof
//     // which is either the req/res pair or the has of the session proof
//     let leaves: Vec<[u8; 32]> = leaves
//         .iter()
//         .map(|proof_response| {
//             let proof_byte_content = proof_response.as_bytes();

//             Sha256::hash(proof_byte_content)
//         })
//         .collect();

//     let merkle_tree = generate_merkle_tree(&leaves);

//     let root = merkle_tree.root().unwrap();
//     let derived_root_hash = hex::encode(root);
//     let root_hash = root_hash.to_owned();

//     // println!("derived_root_hash: {}", derived_root_hash);
//     // println!("root_hash: {}", root_hash);

//     return derived_root_hash == root_hash;
// }

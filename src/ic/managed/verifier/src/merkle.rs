use candid::CandidType;
use serde::Deserialize;

use crate::{proof::ProofResponse, utils::hash};

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct MerkleTree {
    pub nodes: Vec<String>,
    pub num_leaves: usize,
    pub root: String,
}

impl MerkleTree {
    // Function to build the Merkle Tree
    pub fn new(leaves: Vec<String>) -> MerkleTree {
        let num_leaves = leaves.len();
        if num_leaves < 1 {
            panic!("number of leaves should be more than 1")
        }

        let mut nodes: Vec<String> = leaves.iter().map(|leaf| hash(leaf)).collect();
        let mut current_level = nodes.clone();

        // Construct upper levels until we have a single root
        while current_level.len() > 1 {
            let mut next_level = Vec::new();

            for i in (0..current_level.len()).step_by(2) {
                let left = &current_level[i];
                let right = if i + 1 < current_level.len() {
                    &current_level[i + 1]
                } else {
                    left
                };

                let parent_hash = hash(&(left.clone() + right));
                next_level.push(parent_hash);
            }

            current_level = next_level.clone();
            nodes.extend(next_level);
        }

        let root = nodes.clone().last().unwrap().to_owned();

        MerkleTree {
            root,
            nodes,
            num_leaves,
        }
    }
}

impl From<Vec<ProofResponse>> for MerkleTree {
    fn from(leaves: Vec<ProofResponse>) -> Self {
        // gather the leaves from the content of the verified proof
        // which is either the req/res pair or the has of the session proof
        let leaves: Vec<String> = leaves
            .iter()
            .map(|proof_response| proof_response.get_content())
            .collect();

        MerkleTree::new(leaves)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merkle_tree() {
        let data = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ];

        let expected_root =
            String::from("115cbb4775ed495f3d954dfa47164359a97762b40059d9502895def16eed609c");
        let merkle_tree = MerkleTree::new(data);

        // validate the Merkle root
        let merkle_root = merkle_tree.root.to_owned();
        assert_eq!(merkle_root, expected_root);
    }
}

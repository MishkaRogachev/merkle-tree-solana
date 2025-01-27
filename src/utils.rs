use solana_program::{hash::hash, program_error::ProgramError};

/// Hash a single leaf
pub fn hash_leaf(leaf: &[u8]) -> [u8; 32] {
    hash(leaf).to_bytes()
}

/// Hash two nodes to produce their parent node
pub fn hash_nodes(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut combined = Vec::with_capacity(64);
    combined.extend_from_slice(left);
    combined.extend_from_slice(right);
    hash(&combined).to_bytes()
}

/// Build the Merkle tree and return the root
pub fn build_merkle_root(leaves: &[[u8; 32]]) -> Result<[u8; 32], ProgramError> {
    if leaves.is_empty() {
        return Err(crate::errors::MerkleTreeError::EmptyTree.into());
    }

    let mut current_level = leaves.to_vec();
    while current_level.len() > 1 {
        let mut next_level = Vec::new();
        for pair in current_level.chunks(2) {
            let left = pair[0];
            let right = if pair.len() > 1 { pair[1] } else { pair[0] };
            next_level.push(hash_nodes(&left, &right));
        }
        current_level = next_level;
    }

    Ok(current_level[0]) // Root of the tree
}

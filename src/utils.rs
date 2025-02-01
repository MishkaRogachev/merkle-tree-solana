use crate::errors::MerkleTreeError;
use solana_program::{
    hash::{hash, Hash},
    program_error::ProgramError,
};

/// Hash a leaf's bytes and produce a `Hash` (32 bytes).
pub fn hash_leaf(leaf_data: &[u8]) -> Hash {
    Hash::new(&hash(leaf_data).to_bytes())
}

/// Concatenate two 32-byte hashes and hash them again.
fn hash_two_hashes(left: Hash, right: Hash) -> Hash {
    let mut combined = Vec::with_capacity(64);
    combined.extend_from_slice(left.as_ref());
    combined.extend_from_slice(right.as_ref());
    hash_leaf(&combined)
}

/// Build a Merkle root from a list of `Hash` leaves.
pub fn build_merkle_root(leaves: &[Hash]) -> Result<Hash, ProgramError> {
    if leaves.is_empty() {
        return Err(MerkleTreeError::EmptyTree.into());
    }

    let mut current_level = leaves.to_vec();
    while current_level.len() > 1 {
        let mut next_level = Vec::with_capacity((current_level.len() + 1) / 2);
        for pair in current_level.chunks(2) {
            let left = pair[0];
            let right = if pair.len() > 1 { pair[1] } else { pair[0] };
            next_level.push(hash_two_hashes(left, right));
        }
        current_level = next_level;
    }

    Ok(current_level[0])
}

/// Generates a Merkle proof for a specific leaf index.
pub fn build_proof(leaf_index: usize, leaves: &[Hash]) -> Result<Vec<Hash>, ProgramError> {
    if leaves.is_empty() || leaf_index >= leaves.len() {
        return Err(ProgramError::InvalidArgument);
    }

    let mut proof = Vec::new();
    let mut level = leaves.to_vec();
    let mut index = leaf_index;

    while level.len() > 1 {
        let mut next_level = Vec::new();

        for chunk in level.chunks(2) {
            let left = chunk[0];
            let right = if chunk.len() > 1 { chunk[1] } else { chunk[0] };

            if index % 2 == 0 {
                if chunk.len() > 1 {
                    proof.push(right);
                }
            } else {
                proof.push(left);
            }

            next_level.push(hash_two_hashes(left, right));
        }

        level = next_level;
        index /= 2;
    }

    Ok(proof)
}

/// Recomputes the Merkle root from a `leaf_data` and `proof` (list of sibling hashes).
pub fn recompute_merkle_root_from_proof(
    leaf_data: &[u8],
    proof: &[Hash],
) -> Result<Hash, ProgramError> {
    if proof.is_empty() {
        return Err(ProgramError::InvalidArgument);
    }

    // Start with the hashed leaf
    let mut current = hash_leaf(leaf_data);

    // Traverse the proof and compute the Merkle root
    for sibling in proof {
        if current <= *sibling {
            current = hash_two_hashes(current, *sibling);
        } else {
            current = hash_two_hashes(*sibling, current);
        }
    }

    Ok(current)
}

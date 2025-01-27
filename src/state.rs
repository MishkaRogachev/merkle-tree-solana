use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
};

/// Represents the state of the Merkle tree
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MerkleTreeAccount {
    pub is_initialized: bool,  // Whether the tree is initialized
    pub root: [u8; 32],        // The Merkle root
    pub leaves: Vec<[u8; 32]>, // Flattened list of leaf nodes
}

impl Sealed for MerkleTreeAccount {}

impl IsInitialized for MerkleTreeAccount {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for MerkleTreeAccount {
    const LEN: usize = 1024; // Adjust based on maximum tree size (root + leaves)

    fn pack_into_slice(&self, dst: &mut [u8]) {
        dst[0] = self.is_initialized as u8;
        dst[1..33].copy_from_slice(&self.root);

        let leaves_offset = 33;
        for (i, leaf) in self.leaves.iter().enumerate() {
            let start = leaves_offset + i * 32;
            let end = start + 32;
            dst[start..end].copy_from_slice(leaf);
        }
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let is_initialized = src[0] != 0;
        let root = src[1..33]
            .try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?;

        let mut leaves = Vec::new();
        for chunk in src[33..].chunks(32) {
            let mut leaf = [0u8; 32];
            leaf.copy_from_slice(chunk);
            leaves.push(leaf);
        }

        Ok(MerkleTreeAccount {
            is_initialized,
            root,
            leaves,
        })
    }
}

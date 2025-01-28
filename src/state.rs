use solana_program::{
    hash::Hash,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
};

/// Total size allocated for the account.
const MERKLE_TREE_ACCOUNT_LEN: usize = 1024;

/// Number of bytes to store the `is_initialized` flag (1 byte).
const IS_INITIALIZED_SIZE: usize = 1;

/// Size of a Solana `Hash` (32 bytes).
const HASH_SIZE: usize = 32;

/// Offset at which the Merkle root starts (immediately after the init flag).
const ROOT_START: usize = IS_INITIALIZED_SIZE;

/// Offset at which the Merkle root ends.
const ROOT_END: usize = ROOT_START + HASH_SIZE;

/// Represents the state of the Merkle tree.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MerkleTreeAccount {
    /// Whether the tree is initialized.
    pub is_initialized: bool,

    /// The Merkle root (Solana `Hash`).
    pub root: Hash,

    /// Flattened list of leaf node hashes (`Hash`).
    pub leaves: Vec<Hash>,
}

impl Sealed for MerkleTreeAccount {}

impl IsInitialized for MerkleTreeAccount {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for MerkleTreeAccount {
    /// The total space this account must at least occupy.
    const LEN: usize = MERKLE_TREE_ACCOUNT_LEN;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        // 1) Write initialization flag (1 byte).
        dst[0] = self.is_initialized as u8;

        // 2) Write Merkle root (32 bytes).
        let root_bytes = self.root.to_bytes();
        dst[ROOT_START..ROOT_END].copy_from_slice(&root_bytes);

        // 3) Write leaves (32 bytes per leaf).
        let mut offset = ROOT_END;
        for leaf in &self.leaves {
            let leaf_bytes = leaf.to_bytes();
            let leaf_end = offset + HASH_SIZE;

            // Safety check: ensure we don't overrun `dst`.
            if leaf_end > dst.len() {
                break; // or return an error if you prefer
            }
            dst[offset..leaf_end].copy_from_slice(&leaf_bytes);

            offset += HASH_SIZE;
        }
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        // 1) Read the `is_initialized` flag.
        let is_initialized = src.get(0).ok_or(ProgramError::InvalidAccountData)? != &0u8;

        // 2) Read Merkle root (32 bytes).
        let root_slice = src
            .get(ROOT_START..ROOT_END)
            .ok_or(ProgramError::InvalidAccountData)?;
        let root = Hash::new(
            root_slice
                .try_into()
                .map_err(|_| ProgramError::InvalidAccountData)?,
        );

        // 3) Read leaves (32 bytes each).
        let mut leaves = Vec::new();
        let mut offset = ROOT_END;
        while offset < src.len() {
            let leaf_end = offset + HASH_SIZE;
            let chunk = src
                .get(offset..leaf_end)
                .ok_or(ProgramError::InvalidAccountData)?;

            let leaf = Hash::new(
                chunk
                    .try_into()
                    .map_err(|_| ProgramError::InvalidAccountData)?,
            );
            leaves.push(leaf);

            offset += HASH_SIZE;
        }

        Ok(MerkleTreeAccount {
            is_initialized,
            root,
            leaves,
        })
    }
}

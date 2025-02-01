use solana_program::program_error::ProgramError;
use thiserror::Error;

/// Custom errors for the Merkle tree program
#[derive(Error, Debug, Copy, Clone)]
pub enum MerkleTreeError {
    /// Account is already initialized
    #[error("The account is already initialized")]
    AccountAlreadyInitialized,

    /// Invalid instruction data
    #[error("Invalid instruction data provided")]
    InvalidInstructionData,

    /// Empty tree provided during initialization
    #[error("Cannot initialize a tree with no leaves")]
    EmptyTree,

    /// Invalid leaf index
    #[error("Invalid leaf index provided")]
    InvalidLeafIndex,

    /// Invalid proof
    #[error("Invalid proof provided")]
    InvalidProof,
}

impl From<MerkleTreeError> for ProgramError {
    fn from(e: MerkleTreeError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

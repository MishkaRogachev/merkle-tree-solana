pub mod build_proof;
pub mod build_tree;
pub mod verify_proof;

use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, hash::Hash, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instructions::build_proof::process_build_proof;
use crate::instructions::build_tree::process_build_tree;
use crate::instructions::verify_proof::process_verify_proof;

/// Instruction variants for the Merkle tree program.
pub enum MerkleInstruction {
    /// Initializes a Merkle tree with a list of leaf nodes.
    BuildTree { data: Vec<Vec<u8>> },

    /// Generates a Merkle proof for a given leaf index.
    BuildProof { leaf_index: u32 },

    /// Verifies a Merkle proof for a given leaf.
    VerifyProof {
        leaf_data: Vec<u8>,
        proof: Vec<Hash>,
    },
}

impl MerkleInstruction {
    /// Unpacks raw instruction data into a `MerkleInstruction` enum.
    pub fn unpack(instruction_data: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = instruction_data
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        match tag {
            0 => {
                // BuildTree instruction
                let mut data = Vec::new();
                let mut offset = 0;
                while offset < rest.len() {
                    let length = rest[offset] as usize;
                    offset += 1;
                    if offset + length > rest.len() {
                        return Err(ProgramError::InvalidInstructionData);
                    }
                    data.push(rest[offset..offset + length].to_vec());
                    offset += length;
                }
                Ok(MerkleInstruction::BuildTree { data })
            }
            1 => {
                // BuildProof instruction
                if rest.len() != 4 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let leaf_index = u32::from_le_bytes(
                    rest.try_into()
                        .map_err(|_| ProgramError::InvalidInstructionData)?,
                );
                Ok(MerkleInstruction::BuildProof { leaf_index })
            }
            2 => {
                // VerifyProof instruction
                let (leaf_length, rest) = rest
                    .split_first()
                    .ok_or(ProgramError::InvalidInstructionData)?;
                let leaf_length = *leaf_length as usize;
                if rest.len() < leaf_length {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let leaf_data = rest[..leaf_length].to_vec();

                // Deserialize the proof into a vector of `Hash`
                let mut proof = Vec::new();
                let mut offset = leaf_length;
                while offset < rest.len() {
                    if rest[offset..].len() < 32 {
                        return Err(ProgramError::InvalidInstructionData);
                    }
                    let hash_bytes: [u8; 32] = rest[offset..offset + 32]
                        .try_into()
                        .map_err(|_| ProgramError::InvalidInstructionData)?;
                    proof.push(Hash::new(&hash_bytes));
                    offset += 32;
                }

                Ok(MerkleInstruction::VerifyProof { leaf_data, proof })
            }
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = MerkleInstruction::unpack(instruction_data)?;

    match instruction {
        MerkleInstruction::BuildTree { data } => process_build_tree(program_id, accounts, data),
        MerkleInstruction::BuildProof { leaf_index } => {
            process_build_proof(program_id, accounts, leaf_index)
        }
        MerkleInstruction::VerifyProof { leaf_data, proof } => {
            process_verify_proof(program_id, accounts, leaf_data, proof)
        }
    }
}

pub mod build_tree;

use crate::instructions::build_tree::process_build_tree;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

pub enum MerkleInstruction {
    BuildTree { data: Vec<Vec<u8>> },
}

impl MerkleInstruction {
    pub fn unpack(instruction_data: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = instruction_data
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        match tag {
            0 => {
                let mut data = Vec::new();
                let mut offset = 0;
                while offset < rest.len() {
                    let length = rest[offset] as usize;
                    offset += 1;
                    data.push(rest[offset..offset + length].to_vec());
                    offset += length;
                }
                Ok(MerkleInstruction::BuildTree { data })
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
    }
}

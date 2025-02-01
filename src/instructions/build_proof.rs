use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
};

use crate::{errors::MerkleTreeError, state::MerkleTreeAccount, utils::build_proof};

/// Computes a Merkle proof for a given leaf index and returns it via logs.
pub fn process_build_proof(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    leaf_index: u32,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let merkle_account_info = next_account_info(account_info_iter)?;

    // Ensure the account is owned by the program.
    if merkle_account_info.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Load the Merkle tree account.
    let tree_account = MerkleTreeAccount::unpack(&merkle_account_info.try_borrow_data()?)?;

    // Ensure the `leaf_index` is within bounds.
    if leaf_index as usize >= tree_account.leaves.len() {
        return Err(MerkleTreeError::InvalidLeafIndex.into());
    }

    // Generate the proof using the stored leaves.
    let proof = build_proof(leaf_index as usize, &tree_account.leaves)?;

    // Log the proof so the client can extract it.
    for hash in &proof {
        msg!("Proof: {:?}", hash.to_bytes());
    }

    Ok(())
}

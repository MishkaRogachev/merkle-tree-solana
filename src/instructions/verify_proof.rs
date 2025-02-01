use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    hash::Hash,
    msg,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
};

use crate::{
    errors::MerkleTreeError, state::MerkleTreeAccount, utils::recompute_merkle_root_from_proof,
};

/// Verifies a Merkle proof for a given leaf.
pub fn process_verify_proof(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    leaf_data: Vec<u8>,
    proof: Vec<Hash>,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let merkle_account_info = next_account_info(account_info_iter)?;

    // Ensure the account is owned by the program.
    if merkle_account_info.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Load the Merkle tree account.
    let tree_account = MerkleTreeAccount::unpack(&merkle_account_info.try_borrow_data()?)?;

    // Recompute the Merkle root from the proof.
    let recomputed_root = recompute_merkle_root_from_proof(&leaf_data, &proof)?;

    // Compare with the stored root.
    if recomputed_root == tree_account.root {
        msg!("Merkle proof is valid; leaf is in the tree.");
        Ok(())
    } else {
        msg!("Merkle proof is INVALID.");
        Err(MerkleTreeError::InvalidProof.into())
    }
}

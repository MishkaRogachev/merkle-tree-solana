use crate::{
    errors::MerkleTreeError,
    state::MerkleTreeAccount,
    utils::{build_merkle_root, hash_leaf},
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
};

pub fn process_build_tree(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: Vec<Vec<u8>>, // Each inner Vec<u8> is raw leaf data
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let tree_account_info = next_account_info(account_info_iter)?;

    // Ensure this account is owned by the program
    if tree_account_info.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Unpack the existing MerkleTreeAccount
    let mut tree_account = MerkleTreeAccount::unpack(&tree_account_info.try_borrow_data()?)?;

    if tree_account.is_initialized {
        return Err(MerkleTreeError::AccountAlreadyInitialized.into());
    }

    // Convert each leaf to a Solana `Hash`
    let hashed_leaves = data
        .into_iter()
        .map(|bytes| hash_leaf(&bytes))
        .collect::<Vec<_>>();

    // Build the Merkle root from those leaves
    let root = build_merkle_root(&hashed_leaves)?;

    // Set fields and mark as initialized
    tree_account.is_initialized = true;
    tree_account.root = root;
    tree_account.leaves = hashed_leaves;

    // Pack the updated MerkleTreeAccount data back into the account
    MerkleTreeAccount::pack(tree_account, &mut tree_account_info.try_borrow_mut_data()?)?;
    Ok(())
}

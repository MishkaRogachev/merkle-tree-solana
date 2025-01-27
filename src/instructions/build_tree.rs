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
    data: Vec<Vec<u8>>,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let tree_account = next_account_info(account_info_iter)?;

    if tree_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut tree_data = MerkleTreeAccount::unpack(&tree_account.try_borrow_data()?)?;

    if tree_data.is_initialized {
        return Err(MerkleTreeError::AccountAlreadyInitialized.into());
    }

    let hashed_leaves: Vec<[u8; 32]> = data.iter().map(|d| hash_leaf(d)).collect();
    let root = build_merkle_root(&hashed_leaves)?;

    tree_data.is_initialized = true;
    tree_data.root = root;
    tree_data.leaves = hashed_leaves;

    MerkleTreeAccount::pack(tree_data, &mut tree_account.try_borrow_mut_data()?)?;
    Ok(())
}

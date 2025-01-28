use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::str::FromStr;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct MerkleTreeInput {
    pub leaves: Vec<Vec<u8>>,
}

#[tokio::main]
async fn main() {
    // Solana program ID (replace with your deployed program ID)
    let program_id = Pubkey::from_str("4yHHU9UeBPviasoqjm6Vz7y1LbRRc7e3LNRhELTjUvLK").unwrap();

    // Connect to the Solana devnet
    let rpc_url = String::from("http://127.0.0.1:8899");
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // Generate a new keypair for the payer
    let payer = Keypair::new();

    // Request airdrop for the payer
    let airdrop_amount = 1_000_000_000; // 1 SOL
    let signature = client
        .request_airdrop(&payer.pubkey(), airdrop_amount)
        .expect("Failed to request airdrop");

    // Wait for airdrop confirmation
    println!("Waiting for airdrop confirmation...");
    loop {
        if client.confirm_transaction(&signature).unwrap() {
            println!("Airdrop confirmed!");
            break;
        }
    }

    // Create a new keypair for the Merkle tree account
    let tree_account = Keypair::new();

    // Calculate the space required for the Merkle tree account
    let account_space = 1024; // Ensure this matches your `MerkleTreeAccount::LEN`

    // Create the Merkle tree account
    let rent_lamports = client
        .get_minimum_balance_for_rent_exemption(account_space)
        .unwrap();
    let create_account_instruction = system_instruction::create_account(
        &payer.pubkey(),
        &tree_account.pubkey(),
        rent_lamports,
        account_space as u64,
        &program_id,
    );

    // Generate sample transaction IDs as test data
    let tx_ids = vec![
        "f4eecb34d9274d1b99b3d3ccafe8a9e7",
        "a2cbe39281d14b09b344e0334de4a39e",
        "63f8d1e6b1a247baaf9d4e3dcf9e4cb2",
        "fe982d0c48214de09311a73c3c8e17a0",
    ];

    // Convert each string to bytes
    let leaves: Vec<Vec<u8>> = tx_ids
        .into_iter()
        .map(|tx| tx.as_bytes().to_vec())
        .collect();

    // Construct the struct
    let input = MerkleTreeInput { leaves };

    // Call `try_to_vec` directly on the struct
    let mut instruction_data: Vec<u8> = Vec::new();
    input
        .serialize(&mut instruction_data)
        .expect("Failed to serialize");

    // Build the instruction to initialize the Merkle tree
    let init_tree_instruction = Instruction::new_with_borsh(
        program_id,
        &instruction_data, // Serialized leaves as input
        vec![
            AccountMeta::new(tree_account.pubkey(), false),
            AccountMeta::new(payer.pubkey(), true),
        ],
    );

    // Build and send the transaction
    let mut transaction = Transaction::new_with_payer(
        &[create_account_instruction, init_tree_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(
        &[&payer, &tree_account],
        client.get_latest_blockhash().unwrap(),
    );

    // Send and confirm the transaction
    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Transaction successful! Signature: {}", signature),
        Err(err) => eprintln!("Transaction failed: {}", err),
    }

    // Retrieve the account
    let account = client
        .get_account(&tree_account.pubkey())
        .expect("Failed to fetch Merkle tree account");

    // Confirm the account has data
    if account.data.is_empty() {
        eprintln!("Merkle tree account has no data!");
        return;
    }
    println!("Everything is fine");
}

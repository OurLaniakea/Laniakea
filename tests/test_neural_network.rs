use {
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    solana_program_test::*,
    solana_sdk::{signature::Signer, transaction::Transaction},
    std::str::FromStr,
};

#[tokio::test]
async fn test_neural_network_initialization() {
    let program_id = Pubkey::from_str("GNNv1neural1111111111111111111111111111111").unwrap();
    let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
        "galactic_neural_network",
        program_id,
        processor!(process_instruction),
    )
    .start()
    .await;

    let network_account = Keypair::new();
    let authority = Keypair::new();

    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[0], // Initialize instruction
            vec![
                AccountMeta::new(network_account.pubkey(), false),
                AccountMeta::new_readonly(authority.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &authority], recent_blockhash);

    banks_client.process_transaction(transaction).await.unwrap();
}

#[tokio::test]
async fn test_parameter_update() {
    let program_id = Pubkey::from_str("GNNv1neural1111111111111111111111111111111").unwrap();
    let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
        "galactic_neural_network",
        program_id,
        processor!(process_instruction),
    )
    .start()
    .await;

    let network_account = Keypair::new();
    let authority = Keypair::new();

    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[1], // Update parameters instruction
            vec![
                AccountMeta::new(network_account.pubkey(), false),
                AccountMeta::new_readonly(authority.pubkey(), true),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &authority], recent_blockhash);

    banks_client.process_transaction(transaction).await.unwrap();
}

#[tokio::test]
async fn test_neural_computation() {
    let program_id = Pubkey::from_str("GNNv1neural1111111111111111111111111111111").unwrap();
    let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
        "galactic_neural_network",
        program_id,
        processor!(process_instruction),
    )
    .start()
    .await;

    let network_account = Keypair::new();
    let computation_account = Keypair::new();

    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[2], // Neural computation instruction
            vec![
                AccountMeta::new(network_account.pubkey(), false),
                AccountMeta::new(computation_account.pubkey(), false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);

    banks_client.process_transaction(transaction).await.unwrap();
} 
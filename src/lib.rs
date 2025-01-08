use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, instruction::AccountMeta,
    instruction::Instruction, msg, pubkey::Pubkey, sysvar::rent::Rent,
};
use std::io::Write;

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let account = &accounts[1];

    let mut slice = account.try_borrow_mut_data()?;

    let _ = slice.write(&[0xff; 1024]);

    return Ok(());
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;
    use solana_program_test::*;
    use solana_sdk::{
        signature::{Keypair, Signer},
        transaction::Transaction,
    };

    #[tokio::test]
    async fn test_hello_world() {
        let program_id = Pubkey::new_unique();
        let mut p = ProgramTest::new("hello_world", program_id, processor!(process_instruction));
        let (mut banks_client, payer, recent_blockhash) = p.start().await;

        const ACC_SIZE: u64 = 165;
        let new_acc = Keypair::new();
        let create_acc = solana_sdk::system_instruction::create_account(
            &payer.pubkey(),
            &new_acc.pubkey(),
            Rent::default().minimum_balance(1024 as usize),
            ACC_SIZE,
            &program_id,
        );
        let mut transaction = Transaction::new_with_payer(&[create_acc], Some(&payer.pubkey()));
        transaction.sign(&[&payer, &new_acc], recent_blockhash);
        let transaction_result = banks_client.process_transaction(transaction).await;
        assert!(transaction_result.is_ok());

        let call = Instruction::new_with_bincode(
            program_id,
            &(),
            vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(new_acc.pubkey(), false),
                AccountMeta::new_readonly(program_id, false),
            ],
        );
        let mut transaction = Transaction::new_with_payer(&[call], Some(&payer.pubkey()));

        transaction.sign(&[&payer], recent_blockhash);

        // Process the transaction
        let transaction_result = banks_client.process_transaction(transaction).await;
        assert!(transaction_result.is_ok());
        println! {"{:?}", banks_client.get_root_slot().await.unwrap()};

        // Print account data
        let account = banks_client
            .get_account(new_acc.pubkey())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(account.data.len(), 165);
        assert_eq!(account.data, vec![0xff; 165]);
    }
}

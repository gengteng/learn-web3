use solana_client::rpc_client::RpcClient;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{read_keypair_file, Keypair, Signer};
use std::str::FromStr;
use token::instruction::TokenInstruction;

fn main() -> anyhow::Result<()> {
    let rpc_client = RpcClient::new("http://127.0.0.1:8899".to_string());
    let payer = read_keypair_file("/Users/gengteng/.config/solana/test.json").unwrap();
    let program_id = Pubkey::from_str("2jVDHBPbRfsULciH2uiaz77mJkBvLrLc3EhUa6MBG8tM")?;

    let mint_account = Keypair::new();

    println!("Creating mint account: {}", mint_account.pubkey());

    create_token(
        &rpc_client,
        program_id,
        &payer,
        &mint_account,
        payer.pubkey(),
        6,
    )?;

    Ok(())
}

fn create_token(
    rpc_client: &RpcClient,
    program_id: Pubkey,
    payer: &Keypair,
    mint_account: &Keypair,
    mint_authority: Pubkey,
    decimal: u8,
) -> anyhow::Result<()> {
    let token_instruction = TokenInstruction::CreateToken { decimal };
    let data = borsh::to_vec(&token_instruction)?;

    let accounts = vec![
        AccountMeta::new(mint_account.pubkey(), true),
        AccountMeta::new_readonly(mint_authority, false),
        AccountMeta::new_readonly(payer.pubkey(), false),
        AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
        AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
    ];

    let token_instruction = Instruction {
        program_id,
        accounts,
        data,
    };

    let latest_blockhash = rpc_client.get_latest_blockhash()?;
    let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[token_instruction],
        Some(&payer.pubkey()),
        &[payer, mint_account],
        latest_blockhash,
    );

    let signature = rpc_client.send_and_confirm_transaction(&transaction)?;
    println!("{}", signature);
    Ok(())
}

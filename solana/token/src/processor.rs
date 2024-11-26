use crate::instruction::TokenInstruction;
use borsh::BorshDeserialize;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;
use solana_program::{msg, system_instruction};
use spl_token::instruction::initialize_mint;
use spl_token::state::Mint;

pub struct Processor {}

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let token_instruction = TokenInstruction::try_from_slice(instruction_data)?;
        match token_instruction {
            TokenInstruction::CreateToken { decimal } => {
                msg!("Instruction: CreateToken");
                Self::create_token(accounts, decimal)?;
            }
            TokenInstruction::Mint { amount } => {
                msg!("Instruction: Mint");
                Self::mint(accounts, amount)?;
            }
        }
        Ok(())
    }

    fn create_token(accounts: &[AccountInfo], decimal: u8) -> ProgramResult {
        let iter = &mut accounts.iter();
        let mint_account = next_account_info(iter)?;
        let mint_authority = next_account_info(iter)?;
        let payer = next_account_info(iter)?;
        let rent_sysvar = next_account_info(iter)?;
        let system_program = next_account_info(iter)?;
        let token_program = next_account_info(iter)?;

        msg!("Creating mint accout: {}", mint_account.key);

        let create_account_ix = system_instruction::create_account(
            payer.key,
            mint_account.key,
            Rent::get()?.minimum_balance(Mint::LEN),
            Mint::LEN as u64,
            token_program.key,
        );

        invoke(
            &create_account_ix,
            &[
                mint_account.clone(),
                payer.clone(),
                system_program.clone(),
                token_program.clone(),
            ],
        )?;

        let mint_init_ix = initialize_mint(
            token_program.key,
            mint_account.key,
            mint_authority.key,
            None,
            decimal,
        )?;

        invoke_signed(
            &mint_init_ix,
            &[
                mint_account.clone(),
                rent_sysvar.clone(),
                token_program.clone(),
                mint_authority.clone(),
            ],
            &[],
        )?;

        msg!("Mint account created: {}", mint_account.key);
        Ok(())
    }

    fn mint(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
        msg!("Mint");
        Ok(())
    }
}

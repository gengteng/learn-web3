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
use spl_associated_token_account::instruction::create_associated_token_account;
use spl_token::instruction::initialize_mint;
use spl_token::state::Mint;

pub struct Processor {}

impl Processor {
    pub fn process(
        _program_id: &Pubkey,
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
                Self::mint_token(accounts, amount)?;
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

    fn mint_token(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
        let account_iter = &mut accounts.iter();
        let mint_account = next_account_info(account_iter)?;
        let associated_token_account = next_account_info(account_iter)?;
        let rent_sysvar = next_account_info(account_iter)?;
        let payer = next_account_info(account_iter)?;
        let system_program = next_account_info(account_iter)?;
        let token_program = next_account_info(account_iter)?;
        let ata_program = next_account_info(account_iter)?;

        msg!("ATA is: {}", associated_token_account.key);

        if associated_token_account.lamports() == 0 {
            msg!("Creating associated token account");
            let create_account_ix = create_associated_token_account(
                payer.key,
                payer.key,
                mint_account.key,
                token_program.key,
            );

            invoke(
                &create_account_ix,
                &[
                    payer.clone(),
                    associated_token_account.clone(),
                    mint_account.clone(),
                    system_program.clone(),
                    token_program.clone(),
                    rent_sysvar.clone(),
                    ata_program.clone(),
                ],
            )?;
        }

        msg!(
            "Minting {} tokens to ATA {}",
            amount,
            associated_token_account.key
        );

        let mint_ix = spl_token::instruction::mint_to(
            token_program.key,
            mint_account.key,
            associated_token_account.key,
            payer.key,
            &[payer.key],
            amount,
        )?;

        invoke(
            &mint_ix,
            &[
                mint_account.clone(),
                payer.clone(),
                associated_token_account.clone(),
                token_program.clone(),
            ],
        )?;

        msg!(
            "Minted {} tokens to ATA {}",
            amount,
            associated_token_account.key
        );

        Ok(())
    }
}

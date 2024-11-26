pub mod error;
pub mod instruction;

#[cfg(not(feature = "no-entrypoint"))]
pub mod processor;

#[cfg(not(feature = "no-entrypoint"))]
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
use crate::processor::Processor;
#[cfg(not(feature = "no-entrypoint"))]
use solana_program::entrypoint::ProgramResult;
#[cfg(not(feature = "no-entrypoint"))]
use solana_program::{account_info::AccountInfo, entrypoint, pubkey::Pubkey};

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

#[cfg(not(feature = "no-entrypoint"))]
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Processor::process(program_id, accounts, instruction_data)
}

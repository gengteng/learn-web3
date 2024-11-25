use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello, World!");

    msg!("Our program's Program ID: {}", program_id);
    msg!("Accounts: {:?}", accounts);
    msg!("Instruction data: {:?}", instruction_data);
    Ok(())
}
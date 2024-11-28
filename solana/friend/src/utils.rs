use solana_program::account_info::AccountInfo;
use solana_program::program_error::ProgramError;
use solana_program::sysvar::Sysvar;

pub fn get_accounts<'a, 'b, const N: usize>(
    account: &'b [AccountInfo<'a>],
) -> Result<[&'b AccountInfo<'a>; N], ProgramError> {
    if account.len() < N {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    let mut accounts = [&account[0]; N];
    for i in 1..N {
        accounts[i] = &account[i];
    }
    Ok(accounts)
}

pub fn get_timestamp() -> Result<u64, ProgramError> {
    Ok(solana_program::clock::Clock::get()?.unix_timestamp as u64)
}

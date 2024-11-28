use crate::instruction::{SeedType, SocialInstruction};
use crate::state::user::{Post, PostStats, Profile, Space};
use crate::utils::get_accounts;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use solana_program::borsh1::try_from_slice_unchecked;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program::invoke_signed;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = SocialInstruction::try_from_slice(instruction_data)?;
    match instruction {
        SocialInstruction::InitializeUser { seed_type } => {
            process_initialize_user(seed_type, program_id, accounts)
        }
        SocialInstruction::FollowUser { user } => process_follow_user(&user, accounts),
        SocialInstruction::UnfollowUser { user } => process_unfollow_user(&user, accounts),
        SocialInstruction::QueryFollowers => process_query_followers(accounts),
        SocialInstruction::PostContent { content } => process_post_content(&content, accounts),
        SocialInstruction::QueryPosts => process_query_posts(accounts),
    }
}

fn process_initialize_user(
    seed_type: SeedType,
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let [user_account, pda_account, system_program] = get_accounts::<3>(accounts)?;

    msg!("Seed type: {}", seed_type.to_str());

    let (pda, bump) = Pubkey::find_program_address(
        &[user_account.key.as_ref(), seed_type.to_str().as_bytes()],
        program_id,
    );

    msg!("PDA: {}", pda);

    if pda != *pda_account.key {
        return Err(solana_program::program_error::ProgramError::InvalidArgument);
    }

    let rent = Rent::get()?;

    let space = match seed_type {
        SeedType::Profile => Profile::space(),
        SeedType::Post => PostStats::space(),
    };

    let rent_lamports = rent.minimum_balance(space);

    let create_account_ix = solana_program::system_instruction::create_account(
        &user_account.key,
        &pda,
        rent_lamports,
        space as u64,
        program_id,
    );

    invoke_signed(
        &create_account_ix,
        &[
            user_account.clone(),
            pda_account.clone(),
            system_program.clone(),
        ],
        &[&[
            user_account.key.as_ref(),
            seed_type.to_str().as_bytes(),
            &[bump],
        ]],
    )?;

    match seed_type {
        SeedType::Profile => {
            let mut profile = Profile::default();
            profile.serialize(&mut *pda_account.data.borrow_mut())?;
        }
        SeedType::Post => {
            let mut post_stats = PostStats::default();
            post_stats.serialize(&mut *pda_account.data.borrow_mut())?;
        }
    }
    msg!("User account initialized");
    Ok(())
}

fn process_follow_user(user: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let [pda_account] = get_accounts::<1>(accounts)?;
    let mut size = 0;
    {
        let data = pda_account.data.borrow();
        match data.iter().as_slice() {
            [l0, l1, data @ ..] => {
                let count = u16::from_le_bytes([*l0, *l1]);
                size = Profile::calculate_space(count as usize);
                msg!("Profile size: {}", size);
            }
            _ => {
                return Err(solana_program::program_error::ProgramError::InvalidArgument);
            }
        }
    }

    let mut profile = Profile::try_from_slice(&pda_account.data.borrow()[..size])?;
    msg!("Profile: {:?}", profile);
    profile.follow(*user);
    profile.serialize(&mut *pda_account.data.borrow_mut())?;

    Ok(())
}

fn process_unfollow_user(user: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    Ok(())
}

fn process_query_followers(accounts: &[AccountInfo]) -> ProgramResult {
    let [pda_account] = get_accounts::<1>(accounts)?;

    let profile = try_from_slice_unchecked::<Profile>(&pda_account.data.borrow())?;
    msg!("Profile: {:?}", profile);
    Ok(())
}

fn process_post_content(content: &str, accounts: &[AccountInfo]) -> ProgramResult {
    Ok(())
}

fn process_query_posts(accounts: &[AccountInfo]) -> ProgramResult {
    Ok(())
}

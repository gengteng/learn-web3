use crate::state::IBuidlProfile;
use anchor_lang::prelude::*;

pub fn create_profile(ctx: Context<CreateProfile>, display_name: String) -> Result<()> {
    ctx.accounts.profile.display_name = display_name;
    Ok(())
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(init_if_needed,
        payer = payer,
        space = 8 + IBuidlProfile::INIT_SPACE,
        seeds = [IBuidlProfile::SEED_PREFIX, payer.key().as_ref()],
        bump,
    )]
    pub profile: Account<'info, IBuidlProfile>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

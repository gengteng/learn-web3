use anchor_lang::prelude::*;

use crate::state::{IBuidlProfile, Post};

pub fn create_post(ctx: Context<CreatePost>, content: String) -> Result<()> {
    ctx.accounts.post.content = content;
    ctx.accounts.post.author = *ctx.accounts.signer.key;
    ctx.accounts.profile.post_count += 1;
    Ok(())
}

#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [IBuidlProfile::SEED_PREFIX, signer.key().as_ref()],
        bump,
    )]
    pub profile: Account<'info, IBuidlProfile>,

    #[account(init_if_needed,
        payer = signer,
        space = 8 + Post::INIT_SPACE,
        seeds = [Post::SEED_PREFIX, signer.key().as_ref(), (profile.post_count + 1).to_le_bytes().as_ref()],
        bump,
    )]
    pub post: Account<'info, Post>,

    pub system_program: Program<'info, System>,
}

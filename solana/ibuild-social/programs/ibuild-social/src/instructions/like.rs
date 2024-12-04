use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use crate::state::{IBuidlProfile, Like, Post};

pub fn create_like(ctx: Context<CreateLike>) -> Result<()> {
    ctx.accounts
        .like
        .update(ctx.accounts.post.key(), ctx.accounts.profile.key());
    ctx.accounts.post.like_count += 1;

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.author_token_account.to_account_info(),
                authority: ctx.accounts.mint_account.to_account_info(),
            },
            &[&[b"mint", &[ctx.bumps.mint_account]]],
        ),
        100,
    )?;
    Ok(())
}

#[derive(Accounts)]
pub struct CreateLike<'info> {
    #[account(
        mut,
        seeds = [b"mint"],
        bump,
    )]
    pub mint_account: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = author_wallet,
    )]
    pub author_token_account: Account<'info, TokenAccount>,

    /// CHECK: Author wallet
    pub author_wallet: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [IBuidlProfile::SEED_PREFIX, payer.key().as_ref()],
        bump,
    )]
    pub profile: Account<'info, IBuidlProfile>,

    #[account(mut)]
    pub post: Account<'info, Post>,

    #[account(init,
        payer = payer,
        space = 8 + Like::INIT_SPACE,
        seeds = [Like::SEED_PREFIX, profile.key().as_ref(), post.key().as_ref()],
        bump,
    )]
    pub like: Account<'info, Like>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

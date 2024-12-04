use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("E68QqA8wrwZVsg1EVQ6Li4MWFUtw7H3gtJefTDpAx1CD");

#[program]
pub mod ibuild_social {
    use super::*;

    pub fn create_profile(ctx: Context<CreateProfile>, display_name: String) -> Result<()> {
        instructions::profile::create_profile(ctx, display_name)
    }

    pub fn create_post(ctx: Context<CreatePost>, content: String) -> Result<()> {
        instructions::post::create_post(ctx, content)
    }

    pub fn create_like(ctx: Context<CreateLike>) -> Result<()> {
        instructions::like::create_like(ctx)
    }

    pub fn create_token_mint_account(ctx: Context<CreateTokenMintAccount>) -> Result<()> {
        instructions::token::create_token_mint_account(ctx)
    }

    pub fn mint_nft(ctx: Context<MintNft>, nft_id: String) -> Result<()> {
        instructions::nft::mint_nft(ctx, nft_id)
    }

    pub fn nft_stake(ctx: Context<NftStake>) -> Result<()> {
        instructions::stake::nft_stake(ctx)
    }

    pub fn nft_withdraw(ctx: Context<WithdrawNftStake>) -> Result<()> {
        instructions::stake::nft_withdraw(ctx)
    }
}

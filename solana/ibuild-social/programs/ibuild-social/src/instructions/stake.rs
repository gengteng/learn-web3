use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{burn, mint_to, transfer, Burn, Mint, MintTo, Token, TokenAccount, Transfer},
};

use crate::state::stake::StakeInfo;

pub fn nft_stake(ctx: Context<NftStake>) -> Result<()> {
    msg!("Staking NFT");
    ctx.accounts.stake_info.set_inner(StakeInfo::new(
        ctx.accounts.signer.key(), 
        ctx.accounts.nft_mint_account.key(), 
        Clock::get()?.epoch
    ));

    msg!("Transferring NFT");
    // 转移 NFT
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.nft_associated_token_account.to_account_info(),
                to: ctx.accounts.contract_nft_associated_token_account.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            },
        ),
        1,
    )?;

    let signer_seeds: &[&[&[u8]]] = &[&[b"mint", &[ctx.bumps.token_mint_account]]];

    // mint TOKEN
    msg!("Minting TOKEN");
    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.token_mint_account.to_account_info(),
                to: ctx.accounts.token_associated_token_account.to_account_info(),
                authority: ctx.accounts.token_mint_account.to_account_info(),
            },
        ).with_signer(signer_seeds), 
        10000
    )?;

    msg!("NFT staked successfully");
    Ok(())
}


#[derive(Accounts)]
pub struct NftStake<'info> {
    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + StakeInfo::INIT_SPACE,
        seeds = [
            StakeInfo::SEED_PREFIX, 
            nft_mint_account.key().as_ref()
        ],
        bump,
    )]
    pub stake_info: Box<Account<'info, StakeInfo>>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = nft_mint_account,
        associated_token::authority = stake_info,
    )]
    pub contract_nft_associated_token_account: Box<Account<'info, TokenAccount>>,

    /// 确认 TOKEN 是合约的派生 PDA
    #[account(
        mut,
        seeds = [b"mint"],
        bump,
    )]
    pub token_mint_account: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = token_mint_account,
        associated_token::authority = signer,
    )]
    pub token_associated_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub nft_mint_account: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = nft_mint_account,
        associated_token::authority = signer,
    )]
    pub nft_associated_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[error_code]
pub enum Error {
    #[msg("Invalid NFT Mint Account")]
    InvalidNftMintAccount,
    #[msg("Unauthorized")]
    Unauthorized,
}

pub fn nft_withdraw(ctx: Context<WithdrawNftStake>) -> Result<()> {
    // 检查质押关系

    require!(
        ctx.accounts.stake_info.nft_mint == ctx.accounts.nft_mint_account.key(),
        Error::InvalidNftMintAccount
    );

    require!(
        ctx.accounts.stake_info.staker == *ctx.accounts.signer.key,
        Error::Unauthorized
    );

    let nft_mint_account_key = ctx.accounts.nft_mint_account.key();
    let signer_seeds: &[&[&[u8]]] = &[&[
        StakeInfo::SEED_PREFIX,
        nft_mint_account_key.as_ref(), 
        &[ctx.bumps.stake_info]
    ]];

    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.contract_nft_associated_token_account.to_account_info(),
                to: ctx.accounts.nft_associated_token_account.to_account_info(),
                authority: ctx.accounts.stake_info.to_account_info(),
            },
        ).with_signer(signer_seeds),
        1
    )?;

    let epoch = Clock::get()?.epoch;

    burn(CpiContext::new(
            ctx.accounts.token_program.to_account_info(), 
            Burn {
                mint: ctx.accounts.token_mint_account.to_account_info(),
                from: ctx.accounts.token_associated_token_account.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            }
        ), 
        ctx.accounts.stake_info.salvage_value(epoch, 10000)
    )?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawNftStake<'info> {
    #[account(
        mut,
        seeds = [
            StakeInfo::SEED_PREFIX, 
            nft_mint_account.key().as_ref()
        ],
        bump,
    )]
    pub stake_info: Box<Account<'info, StakeInfo>>,

    #[account(
        mut,
        associated_token::mint = nft_mint_account,
        associated_token::authority = stake_info,
    )]
    pub contract_nft_associated_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"mint"],
        bump,
    )]
    pub token_mint_account: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = token_mint_account,
        associated_token::authority = signer,
    )]
    pub token_associated_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub nft_mint_account: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = nft_mint_account,
        associated_token::authority = signer,
    )]
    pub nft_associated_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

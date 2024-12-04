use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
        Metadata,
    },
    token::{Mint, Token},
};

pub fn create_token_mint_account(ctx: Context<CreateTokenMintAccount>) -> Result<()> {
    // check if the metadata account is initialized
    // if not, initialize it

    if !ctx.accounts.metadata_account.data_is_empty() {
        return Ok(());
    }

    let signer_seeds: &[&[&[u8]]] = &[&[b"mint", &[ctx.bumps.mint_account]]];

    let ctx = CpiContext::new_with_signer(
        ctx.accounts.token_metadata_program.to_account_info(),
        CreateMetadataAccountsV3 {
            metadata: ctx.accounts.metadata_account.to_account_info(),
            mint: ctx.accounts.mint_account.to_account_info(),
            mint_authority: ctx.accounts.mint_account.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            update_authority: ctx.accounts.mint_account.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        },
        signer_seeds,
    );

    let data_v2 = DataV2 {
        name: "ggtt".to_string(),
        symbol: "GT".to_string(),
        uri: "https://gteng.org".to_string(),
        seller_fee_basis_points: 500,
        creators: None,
        collection: None,
        uses: None,
    };

    create_metadata_accounts_v3(ctx, data_v2, false, true, None)?;
    Ok(())
}

#[derive(Accounts)]
pub struct CreateTokenMintAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        seeds = [b"mint"],
        bump,
        mint::decimals = 2,
        mint::authority = mint_account.key(),
    )]
    pub mint_account: Account<'info, Mint>,

    /// CHECK: Validate the metadata account is initialized
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            mint_account.key().as_ref(),
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>,
}

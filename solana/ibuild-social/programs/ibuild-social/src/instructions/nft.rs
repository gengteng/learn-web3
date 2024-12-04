use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3,
        mpl_token_metadata::{
            instructions::{CreateV1CpiBuilder, MintV1CpiBuilder},
            types::DataV2,
        },
        CreateMasterEditionV3, CreateMetadataAccountsV3, Metadata,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

pub fn mint_nft(ctx: Context<MintNft>, nft_id: String) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[b"nft", nft_id.as_bytes(), &[ctx.bumps.nft_mint_account]]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_metadata_program.to_account_info(),
        CreateMetadataAccountsV3 {
            metadata: ctx.accounts.metadata_account.to_account_info(),
            mint: ctx.accounts.nft_mint_account.to_account_info(),
            mint_authority: ctx.accounts.nft_mint_account.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            update_authority: ctx.accounts.nft_mint_account.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        },
        signer_seeds,
    );

    let data_v2 = DataV2 {
        name: format!("GTT#{}", nft_id),
        symbol: "GTT".to_string(),
        uri: "https://gteng.org".to_string(),
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    create_metadata_accounts_v3(cpi_ctx, data_v2, false, true, None)?;

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.nft_mint_account.to_account_info(),
            to: ctx.accounts.nft_associated_token_account.to_account_info(),
            authority: ctx.accounts.nft_mint_account.to_account_info(),
        },
        signer_seeds,
    );
    mint_to(cpi_ctx, 1)?;

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        CreateMasterEditionV3 {
            edition: ctx.accounts.master_edition_account.to_account_info(),
            mint: ctx.accounts.nft_mint_account.to_account_info(),
            update_authority: ctx.accounts.nft_mint_account.to_account_info(),
            mint_authority: ctx.accounts.nft_mint_account.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            metadata: ctx.accounts.metadata_account.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        },
        signer_seeds,
    );
    create_master_edition_v3(cpi_ctx, Some(1))?;

    Ok(())
}

pub fn mint_nft_ex(ctx: Context<MintNft>, nft_id: String) -> Result<()> {
    let signers_seeds: &[&[&[u8]]] = &[&[b"nft", nft_id.as_bytes(), &[ctx.bumps.nft_mint_account]]];

    CreateV1CpiBuilder::new(&ctx.accounts.token_metadata_program)
        .mint(ctx.accounts.nft_mint_account.as_ref(), true)
        .metadata(&ctx.accounts.metadata_account)
        .master_edition(Some(&ctx.accounts.master_edition_account))
        .invoke_signed(signers_seeds)?;

    MintV1CpiBuilder::new(&ctx.accounts.token_program)
        .mint(ctx.accounts.nft_mint_account.as_ref())
        .invoke_signed(signers_seeds)?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(nft_id: String)]
pub struct MintNft<'info> {
    #[account(
        mut,
        seeds = [
            b"metadata", 
            token_metadata_program.key().as_ref(), 
            nft_mint_account.key().as_ref(),
            b"edition",
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    /// CHECK: Validate address by deriving from seeds
    pub master_edition_account: UncheckedAccount<'info>,

    #[account(
        init,
        payer = payer,
        seeds = [b"nft", nft_id.as_bytes()],
        bump,
        mint::decimals = 0,
        mint::authority = nft_mint_account.key(),
        mint::freeze_authority = nft_mint_account.key(),
    )]
    pub nft_mint_account: Account<'info, Mint>,

    /// CHECK: Validate the metadata account is initialized
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            nft_mint_account.key().as_ref(),
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = nft_mint_account,
        associated_token::authority = payer,
    )]
    pub nft_associated_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

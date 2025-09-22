use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use mpl_token_metadata::{types::TokenStandard, instructions::CreateV1CpiBuilder};

const METADATA_PROGRAM_ID: Pubkey = pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

pub fn create_metadata(
    ctx: Context<CreateMetadata>,
    name: String,
    symbol: String,
    uri: String,
) -> Result<()> {
    msg!("Creating metadata for token: {}", name);

    // Create the metadata using Metaplex CPI
    let cpi_program = ctx.accounts.token_metadata_program.to_account_info();
    
    let mut cpi = CreateV1CpiBuilder::new(&cpi_program);
    
    let binding: AccountInfo<'_> = ctx.accounts.mint.to_account_info();
    cpi.metadata(&ctx.accounts.metadata)
        .mint(&binding, true)
        .authority(&ctx.accounts.mint_authority)
        .payer(&ctx.accounts.payer)
        .update_authority(&ctx.accounts.mint_authority, true)
        .system_program(&ctx.accounts.system_program)
        .sysvar_instructions(&ctx.accounts.sysvar_instructions)
        .spl_token_program(Some(&ctx.accounts.token_program))
        .name(name)
        .symbol(symbol)
        .uri(uri)
        .seller_fee_basis_points(0)
        .primary_sale_happened(false)
        .is_mutable(true)
        .token_standard(TokenStandard::Fungible);

    // Use signer seeds for the mint authority (PDA)
    let signer_seeds: &[&[&[u8]]] = &[&[b"mint", &[ctx.bumps.mint]]];
    
    cpi.invoke_signed(signer_seeds)?;

    msg!("Metadata created successfully");
    Ok(())
}

#[derive(Accounts)]
pub struct CreateMetadata<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The mint account (PDA)
    #[account(
        mut,
        seeds = [b"mint"],
        bump,
    )]
    pub mint: Account<'info, Mint>,

    /// Mint authority (same as mint PDA)
    /// CHECK: This is the mint authority PDA
    #[account(
        seeds = [b"mint"],
        bump,
    )]
    pub mint_authority: AccountInfo<'info>,

    /// Metadata account PDA
    /// CHECK: This PDA is checked by Metaplex
    #[account(
        mut
    )]
    pub metadata: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    /// CHECK: Sysvar instructions account
    #[account(address = anchor_lang::solana_program::sysvar::instructions::ID)]
    pub sysvar_instructions: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,

    /// CHECK: Metaplex Token Metadata program
    #[account(address = METADATA_PROGRAM_ID)]
    pub token_metadata_program: AccountInfo<'info>,
}
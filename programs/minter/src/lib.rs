use std::str::FromStr;

use anchor_lang::{accounts::signer, prelude::*};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Burn, Mint, MintTo, Token, TokenAccount, Transfer},
};
use mpl_token_metadata::{accounts::Metadata, types::TokenStandard};

use mpl_token_metadata::types::{DataV2, Creator};
use mpl_token_metadata::instructions::{CreateV1CpiBuilder,UpdateV1CpiBuilder};
use mpl_token_metadata::ID as METADATA_PROGRAM_IDD;
const METADATA_PROGRAM_ID: Pubkey =
    pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

declare_id!("HvstA1bisTEv9iw8jnmFd3YKAqkMQ8RQd5BoR7z2gBMf");

#[program]
pub mod minter {

    use super::*;

    pub fn initialize(ctx: Context<CreateMint>, oracle: Pubkey) -> Result<()> {
        msg!("Minter Program Initialized");
        ctx.accounts.operation.admin = ctx.accounts.signer.key();
        ctx.accounts.operation.oracle = oracle;
        Ok(())
    }

    pub fn update_oracle(ctx: Context<UpdateOperation>, oracle: Pubkey) -> Result<()> {
        let ctx = &mut ctx.accounts.operation;
        ctx.oracle = oracle;
        msg!("Oracle updated to: {}", oracle);
        Ok(())
    }

    pub fn update_admin(ctx: Context<UpdateOperation>, admin: Pubkey) -> Result<()> {
        let ctx = &mut ctx.accounts.operation;
        ctx.admin = admin;
        msg!("Admin updated to: {}", admin);
        Ok(())
    }

    pub fn mint_tokens(ctx: Context<TokenOperations>, amount: u64) -> Result<()> {
        msg!("Minting {} tokens", amount);

        // require!(
        //     ctx.accounts.oracle.key() == ctx.accounts.operation.oracle,
        //     ErrorCode::UnauthorizedOracle
        // );

        let seeds: &[&[u8]] = &[b"oracle_data"]; // same seeds used by the external program
        let (expected_pda, _bump) = Pubkey::find_program_address(seeds, ctx.accounts.oracle_program.key);
        msg!("Expected PDA: {}", expected_pda);
        msg!("Oracle Key: {}", ctx.accounts.oracle_pda.key());
        require_keys_eq!(
            ctx.accounts.oracle_pda.to_account_info().key(),
            expected_pda,
            ErrorCode::UnauthorizedOracle
        );  
 
        let signer_seeds: &[&[&[u8]]] = &[&[b"mint", &[ctx.bumps.mint]]];

        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.mint.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(signer_seeds);

        token::mint_to(cpi_context, amount)?;
        Ok(())
    }

    pub fn burn_tokens(ctx: Context<TokenOperations>, amount: u64) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[b"token", &[ctx.bumps.token_account]]];
        msg!("oracle is {}", ctx.accounts.oracle_pda.key());
        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.token_account.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(signer_seeds);

        token::burn(cpi_context, amount)?;
        Ok(())
    }
    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[b"token", &[ctx.bumps.token_account]]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.token_account.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.token_account.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(signer_seeds);

        token::transfer(cpi_context, amount)?;
        Ok(())
    }
    pub fn get_stats_supply(ctx: Context<TokenOperations>) -> Result<u64> {
        Ok(ctx.accounts.mint.supply)
    }

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
    
    let binding = ctx.accounts.mint.to_account_info();
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



}

#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        seeds = [b"operation"],
        bump,
        payer = signer,
        space = 8 + 32 + 32
    )]
    pub operation: Account<'info, Operation>,

    /// CHECK: This is a mint account that will be created by the token program
    #[account(
        init,
        payer = signer,
        mint::decimals = 6,
        mint::authority = mint,
        mint::freeze_authority = mint,
        seeds = [b"mint"],
        bump
    )]
    pub mint: Account<'info, Mint>,

    /// CHECK: This is a token account that will be created by the token program
    #[account(
        init,
        payer = signer,
        token::mint = mint,
        token::authority = token_account,
        token::token_program = token_program,
        seeds = [b"token"],
        bump
    )]
    pub token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct TokenOperations<'info> {
    #[account(mut)]
    pub oracle_pda: Signer<'info>,

    #[account(
        mut,
        seeds = [b"operation"],
        
        bump
    )]
    pub operation: Account<'info, Operation>,

    /// CHECK: This is a token account controlled by a PDA
    #[account(
        mut,
        seeds = [b"token"],
        bump
    )]
    pub token_account: Account<'info, TokenAccount>,

    /// CHECK: This is a mint account controlled by a PDA
    #[account(
        mut,
        seeds = [b"mint"],
        bump,
    )]
    pub mint: Account<'info, Mint>,
    /// CHECK: Oracle program account info
    pub oracle_program: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"operation"],
        constraint = operation.admin == admin.key() @ ErrorCode::UnauthorizedAdminUser,
        bump
    )]
    pub operation: Account<'info, Operation>,

    #[account(
        mut,
        seeds = [b"token"],
        bump
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"mint"],
        bump,
    )]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub recipient_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BurnTokens<'info> {
    #[account(mut)]
    pub oracle: Signer<'info>,

    #[account(
        mut,
        seeds = [b"operation"],
         has_one = oracle ,
        bump
    )]
    pub operation: Account<'info, Operation>,

    /// CHECK: This is a token account controlled by a PDA
    #[account(
        mut,
        seeds = [b"token"],
        bump
    )]
    pub token_account: Account<'info, TokenAccount>,

    /// CHECK: This is a mint account controlled by a PDA
    #[account(
        mut,
        seeds = [b"mint"],
        bump,
    )]
    pub mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}



#[derive(Accounts)]
pub struct GetStats<'info> {


    #[account(  
        seeds = [b"operation"],
        bump
    )]
    pub operation: Account<'info, Operation>,

    #[account(
        seeds = [b"token"],
        bump
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"mint"],
        bump,
    )]
    pub mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct UpdateOperation<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"operation"],
        constraint = operation.admin == admin.key() @ ErrorCode::UnauthorizedAdminUser,
        bump,
    )]
    pub operation: Account<'info, Operation>,
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

#[account]
pub struct Operation {
    pub admin: Pubkey,
    pub oracle: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized oracle program")]
    UnauthorizedOracle,
     #[msg("Unauthorized Admin user")]
    UnauthorizedAdminUser,
     #[msg("Metadata PDA is invalid")]
    InvalidMetadataPDA,
}

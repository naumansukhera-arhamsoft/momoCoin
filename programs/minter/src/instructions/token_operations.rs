use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Burn, Mint, MintTo, Token, TokenAccount, Transfer},
};
use crate::{state::Operation, errors::ErrorCode};

pub fn mint_tokens(ctx: Context<TokenOperations>, amount: u64) -> Result<()> {

    // require!(
    //     ctx.accounts.oracle.key() == ctx.accounts.operation.oracle,
    //     ErrorCode::UnauthorizedOracle
    // );
    match ctx.accounts.operation.status {
        1 => { }
        0 => return Err(ErrorCode::OperationPaused.into()),
        2 => return Err(ErrorCode::OperationFreezed.into()),
        _ => return Err(ErrorCode::OperationFreezed.into()), // fallback
    }

    msg!("Minting {} tokens", amount);

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
     match ctx.accounts.operation.status {
        1 => { }
        0 => return Err(ErrorCode::OperationPaused.into()),
        2 => return Err(ErrorCode::OperationFreezed.into()),
        _ => return Err(ErrorCode::OperationFreezed.into()), // fallback
    }
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

    match ctx.accounts.operation.status {
        2 => return Err(ErrorCode::OperationFreezed.into()),
        _ => {} // covers all other u8 values
    }

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
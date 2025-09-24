
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ self, Burn, Mint, MintTo, Token, TokenAccount, Transfer },
};
use crate::{
    constants::constants::{ MINT_SEED, OPERATION_SEED, ORACLE_DATA_SEED, TOKEN_ACCOUNT_SEED },
    errors::ErrorCode,
    state::Operation,
};

#[derive(Accounts)]
pub struct TokenOperations<'info> {
    #[account(mut,
        seeds = [ORACLE_DATA_SEED.as_ref()], 
        bump,
        seeds::program = operation.oracle
    )]
    pub oracle_pda: Signer<'info>,

    #[account(
        mut,
        seeds = [OPERATION_SEED.as_ref()],
        bump
    )]
    pub operation: Account<'info, Operation>,

    /// CHECK: This is a token account controlled by a PDA
    #[account(
        mut,
        seeds = [TOKEN_ACCOUNT_SEED.as_ref()],
        bump
    )]
    pub token_account: Account<'info, TokenAccount>,

    /// CHECK: This is a mint account controlled by a PDA
    #[account(
        mut,
        seeds = [MINT_SEED.as_ref()],
        bump,
    )]
    pub mint: Account<'info, Mint>,
    /// CHECK: Oracle program account info
    #[account(address = operation.oracle)]
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
        seeds = [OPERATION_SEED.as_ref()],
        constraint = operation.admin == admin.key() @ ErrorCode::UnauthorizedAdminUser,
        bump
    )]
    pub operation: Account<'info, Operation>,

    #[account(
        mut,
        seeds = [TOKEN_ACCOUNT_SEED.as_ref()],
        bump
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [MINT_SEED.as_ref()],
        bump,
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = recipient_token_account.mint == mint.key()
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetStats<'info> {
    #[account(seeds = [OPERATION_SEED.as_ref()], bump)]
    pub operation: Account<'info, Operation>,

    #[account(seeds = [TOKEN_ACCOUNT_SEED.as_ref()], bump)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(seeds = [MINT_SEED.as_ref()], bump)]
    pub mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn mint_tokens(ctx: Context<TokenOperations>, amount: u64) -> Result<()> {
    match ctx.accounts.operation.status {
        1 => {}
        0 => {
            return Err(ErrorCode::OperationPaused.into());
        }
        _ => {
            return Err(ErrorCode::OperationFreezed.into());
        } // fallback
    }

    msg!("Minting {} tokens", amount);

    let seeds: &[&[u8]] = &[ORACLE_DATA_SEED.as_ref()]; // same seeds used by the external program
    let (expected_pda, _bump) = Pubkey::find_program_address(
        seeds,
        ctx.accounts.oracle_program.key
    );
    msg!("Expected PDA: {}", expected_pda);
    msg!("Oracle Key: {}", ctx.accounts.oracle_pda.key());
    require_keys_eq!(
        ctx.accounts.oracle_pda.to_account_info().key(),
        expected_pda,
        ErrorCode::UnauthorizedOracle
    );

    let signer_seeds: &[&[&[u8]]] = &[&[MINT_SEED.as_ref(), &[ctx.bumps.mint]]];

    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.mint.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(signer_seeds);

    token::mint_to(cpi_context, amount)?;
    ctx.accounts.operation.last_minted = Clock::get()?.unix_timestamp;
    Ok(())
}

pub fn burn_tokens(ctx: Context<TokenOperations>, amount: u64) -> Result<()> {
    match ctx.accounts.operation.status {
        1 => {}
        0 => {
            return Err(ErrorCode::OperationPaused.into());
        }
        _ => {
            return Err(ErrorCode::OperationFreezed.into());
        } // fallback
    }
    let signer_seeds: &[&[&[u8]]] = &[&[TOKEN_ACCOUNT_SEED.as_ref(), &[ctx.bumps.token_account]]];
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
        2 => {
            return Err(ErrorCode::OperationFreezed.into());
        }
        _ => {} // covers all other u8 values
    }
    let current_time = Clock::get()?.unix_timestamp;
    let time_since_last_mint = current_time - ctx.accounts.operation.last_minted;
    if time_since_last_mint < ctx.accounts.operation.cool_down_period as i64 {
        return Err(ErrorCode::CoolDownPeriodNotElapsed.into());
    }

    let signer_seeds: &[&[&[u8]]] = &[&[TOKEN_ACCOUNT_SEED.as_ref(), &[ctx.bumps.token_account]]];

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

pub fn get_stats_supply(ctx: Context<GetStats>) -> Result<u64> {
    Ok(ctx.accounts.mint.supply.into())
}

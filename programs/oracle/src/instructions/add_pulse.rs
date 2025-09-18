use anchor_lang::prelude::*;
use anchor_lang::solana_program::account_info::AccountInfo;
use anchor_spl::token::{Mint, Token, TokenAccount};
use minter::{self, cpi::accounts::TokenOperations, cpi::mint_tokens, program::Minter, cpi::burn_tokens};
use crate::state::*;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct OracleAccount<'info> {
    #[account(
        init,
        seeds = [b"oracle".as_ref(), &(oracle_data_account.latest_pulse + 1).to_string().as_bytes()],
        bump,
        space = 8 + OraclePulse::INIT_SPACE,
        payer = admin,
    )]
    pub oracle_pulse: Account<'info, OraclePulse>,

    #[account(
        mut,
        seeds = [b"oracle_data".as_ref()],
        bump,
    )]
    pub oracle_data_account: Account<'info, OracleData>,

    /// CHECK: This is the minter's operation program account that will be passed to minter
    #[account(mut)]
    pub operation: AccountInfo<'info>,
    
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
    
    /// CHECK: This is the oracle program account that will be passed to minter
    #[account(address = crate::ID)]
    pub oracle_program: AccountInfo<'info>,

    #[account(address = minter::ID)]
    pub minter_program: Program<'info, Minter>,

    #[account(mut)]
    pub admin: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<OracleAccount>, available_bank_balance: u64) -> Result<String> {
    let oracle_pulse: &mut Account<'_, OraclePulse> = &mut ctx.accounts.oracle_pulse;
    let oracle_data_account: &mut Account<'_, OracleData> =
        &mut ctx.accounts.oracle_data_account;

    let bump = ctx.bumps.oracle_data_account;
    let seeds: &[&[u8]] = &[b"oracle_data", &[bump]];
    let signer_seeds = &[&seeds[..]];

    // double check admin
    require!(
        oracle_data_account.admin == ctx.accounts.admin.key(),
        ErrorCode::UnAuthorizedUser
    );

    let timestamp = Clock::get()?.unix_timestamp;
    // safely increment counter
    let current_pulse = oracle_data_account
        .latest_pulse
        .checked_add(1)
        .ok_or(ErrorCode::Overflow)?;

    oracle_data_account.latest_pulse = current_pulse;
    oracle_data_account.last_updated = timestamp;
    oracle_pulse.available_bank_balance = available_bank_balance;
    oracle_pulse.pulse = current_pulse;
    oracle_pulse.timestamp = timestamp;

    msg!(
        "Pulse recorded at: {}, ProgramID: {:?}",
        timestamp,
        ctx.program_id
    );

    let supply = ctx.accounts.mint.supply;
    let token_account_balance = ctx.accounts.token_account.amount;

    msg!("Current supply is: {}", supply);
    msg!("Available balance is: {}", available_bank_balance);
    msg!("Token account balance is: {}", token_account_balance);

    if available_bank_balance > supply {
        let difference = available_bank_balance
            .checked_sub(supply)
            .ok_or(ErrorCode::Overflow)?;
        let mint_amount = difference;
        msg!("Need to mint: {} tokens", mint_amount);
        msg!("Minting new tokens to sync with available balance");

        let cpi_accounts = TokenOperations {
            oracle_pda: ctx.accounts.oracle_data_account.to_account_info(),
            operation: ctx.accounts.operation.to_account_info(),
            token_account: ctx.accounts.token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            oracle_program: ctx.accounts.oracle_program.clone(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };

        let cpi_program: AccountInfo<'_> = ctx.accounts.minter_program.to_account_info();
        let cpi_ctx: CpiContext<'_, '_, '_, '_, TokenOperations<'_>> =
            CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        match mint_tokens(cpi_ctx, mint_amount) {
            Ok(_) => msg!("Minted {} tokens successfully", mint_amount),
            Err(e) => {
                msg!("Failed to mint tokens: {:?}", e);
                ctx.accounts.oracle_pulse.token_operation_log = format!("Failed to mint tokens: {:?}", e);
                ctx.accounts.oracle_pulse.token_operation_sucess = 0;
                ctx.accounts.oracle_pulse.token_operation_type = 1; // mint
            }
            
        }
    } else if available_bank_balance < supply {
        let difference = supply
            .checked_sub(available_bank_balance)
            .ok_or(ErrorCode::Overflow)?;
        let mint_amount = difference;
        msg!("Need to burn: {} tokens", mint_amount);
        msg!("Burning new tokens to sync with available balance");

        let cpi_accounts: TokenOperations<'_> = TokenOperations {
            oracle_pda: ctx.accounts.oracle_data_account.to_account_info(),
            operation: ctx.accounts.operation.to_account_info(),
            token_account: ctx.accounts.token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            oracle_program: ctx.accounts.oracle_program.clone(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };

        let cpi_program: AccountInfo<'_> = ctx.accounts.minter_program.to_account_info();
        let cpi_ctx: CpiContext<'_, '_, '_, '_, TokenOperations<'_>> =
            CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        burn_tokens(cpi_ctx, mint_amount)?;
    } else {
        msg!("No operation needed, supply matches available balance");
        return Ok("Pulse added successfully".to_string());
    }

    Ok("Pulse added successfully".to_string())
}
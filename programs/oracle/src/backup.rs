use anchor_lang::prelude::*;
use anchor_lang::solana_program::account_info::AccountInfo;
use anchor_spl::token::{Mint, MintTo, Token, TokenAccount, Transfer};
use minter::{self, cpi::accounts::TokenOperations, cpi::mint_tokens, program::Minter};
declare_id!("9pCtMqRGA94nX4eaPHPayTBnGqCU9mSzx6v9bN3SX5gH");

#[program]
pub mod oracle {

    use minter::cpi::burn_tokens;

    use super::*;

    pub fn initialize_oracle(ctx: Context<InitOracleData>) -> Result<()> {
        let data = &mut ctx.accounts.oracle_data_account;
        data.latest_pulse = 0;
        data.last_updated = 0;
        data.admin = ctx.accounts.user.key();
        Ok(())
    }

    pub fn add_pulse(ctx: Context<OracleAccount>, available_bank_balance: u64) -> Result<String> {
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
        // let decimals = ctx.accounts.mint.decimals;
        // let factor = 10u64
        //     .checked_pow(decimals as u32)
        //     .ok_or(ErrorCode::Overflow)?;
        let supply = ctx.accounts.mint.supply;
        //     .checked_div(factor)
        //     .ok_or(ErrorCode::Overflow)?;

        let token_account_balance = ctx.accounts.token_account.amount;
        // .checked_div(factor)
        // .ok_or(ErrorCode::Overflow)?;
        msg!("Current supply is: {}", supply);
        msg!("Available balance is: {}", available_bank_balance);
        msg!("Token account balance is: {}", token_account_balance);

        if available_bank_balance > supply {
            let difference = available_bank_balance
                .checked_sub(supply)
                .ok_or(ErrorCode::Overflow)?;
            let mint_amount = difference; //.checked_mul(factor).ok_or(ErrorCode::Overflow)?;
            msg!("Need to mint: {} tokens", mint_amount);
            msg!("Minting new tokens to sync with available balance");

            let cpi_accounts = TokenOperations {
                oracle_pda: ctx.accounts.oracle_data_account.to_account_info(),
                operation: ctx.accounts.operation.to_account_info(),
                token_account: ctx.accounts.token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                oracle_program: ctx.accounts.oracle_program.clone(), // <--- current program id
                associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            };

            let cpi_program: AccountInfo<'_> = ctx.accounts.minter_program.to_account_info();
            let cpi_ctx: CpiContext<'_, '_, '_, '_, TokenOperations<'_>> =
                CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

            mint_tokens(cpi_ctx, mint_amount)?;
        } else if available_bank_balance < supply {
            let difference = supply
                .checked_sub(available_bank_balance)
                .ok_or(ErrorCode::Overflow)?;
            let mint_amount = difference;//.checked_mul(factor).ok_or(ErrorCode::Overflow)?;
            msg!("Need to burn: {} tokens", mint_amount);
            msg!("Burning new tokens to sync with available balance");

            let cpi_accounts: TokenOperations<'_> = TokenOperations {
                oracle_pda: ctx.accounts.oracle_data_account.to_account_info(),
                operation: ctx.accounts.operation.to_account_info(),
                token_account: ctx.accounts.token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                oracle_program: ctx.accounts.oracle_program.clone(), // <--- current program id
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
}

#[derive(Accounts)]
pub struct OracleAccount<'info> {
    #[account(
        init,
        seeds = [b"oracle".as_ref(), &(oracle_data_account.latest_pulse + 1).to_string().as_bytes()],
        bump,
        space = 8 + 32, // Fixed: 8 (discriminator) + 24 (data) = 32 bytes total
        payer = admin,
    )]
    pub oracle_pulse: Account<'info, OraclePulse>,

    #[account(
        mut,
        seeds = [b"oracle_data".as_ref()],
        bump,
    )]
    pub oracle_data_account: Account<'info, OracleData>,

    /// CHECK: will be verified in CPI
    #[account(mut)]
    pub operation: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info,Token>,
    pub associated_token_program:Program<'info, anchor_spl::associated_token::AssociatedToken>,
    /// CHECK: This is the oracle program account that will be passed to minter
    #[account(address = crate::ID)]
    pub oracle_program: AccountInfo<'info>,

    #[account(address = minter::ID)]
    pub minter_program: Program<'info, Minter>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct OracleData {
    pub latest_pulse: u64, // 8 bytes
    pub admin: Pubkey,     // 32 bytes
    pub last_updated: i64, // 8 bytes
                           // Total data: 48 bytes + 8 bytes discriminator = 56 bytes
}

#[account]
pub struct OraclePulse {
    /// Available bank balance, stored in base units (8 decimals).
    pub available_bank_balance: u64, // 8 bytes
    pub pulse: u64, // 8 bytes
    pub timestamp: i64, // 8 bytes
                    // Total data: 24 bytes + 8 bytes discriminator = 32 bytes
}

#[derive(Accounts)]
pub struct InitOracleData<'info> {
    #[account(
        init,
        seeds = [b"oracle_data".as_ref()],
        bump,
        space = 8 + 48, // Fixed: 8 (discriminator) + 48 (data) = 56 bytes total
        payer = user,
    )]
    pub oracle_data_account: Account<'info, OracleData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Integer overflowed")]
    Overflow,
    #[msg("Unauthorized user attempted to modify oracle data")]
    UnAuthorizedUser,
}

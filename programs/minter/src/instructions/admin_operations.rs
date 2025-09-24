use anchor_lang::prelude::*;
use crate::{constants::constants::OPERATION_SEED, errors::ErrorCode, state::Operation};

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
pub fn update_cool_down_period_in_seconds(ctx: Context<UpdateOperation>, time:u64) -> Result<()> {
    let ctx = &mut ctx.accounts.operation;
    ctx.cool_down_period = time;
    msg!("Cool down period updated to: {} seconds", time);
    Ok(())
}
pub fn update_status(ctx: Context<UpdateOperation>, status:u8) -> Result<()> {
    let ctx = &mut ctx.accounts.operation;
    if status != 1 && status != 0 {
        return Err(ErrorCode::InvalidStatus.into());
    }
    msg!("Status updated to: {}", status);
    ctx.status = status;
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateOperation<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [OPERATION_SEED.as_ref()],
        constraint = operation.admin == admin.key() @ ErrorCode::UnauthorizedAdminUser,
        bump,
    )]
    pub operation: Account<'info, Operation>,
}
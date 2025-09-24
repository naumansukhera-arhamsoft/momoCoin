use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount},
};
use crate::{constants::constants::{MINT_SEED, OPERATION_SEED, TOKEN_ACCOUNT_SEED}, state::Operation};

pub fn initialize(ctx: Context<CreateMint>, oracle: Pubkey) -> Result<()> {
    msg!("Minter Program Initialized");
    ctx.accounts.operation.admin = ctx.accounts.signer.key();
    ctx.accounts.operation.oracle = oracle;
    ctx.accounts.operation.status = 0; // paused
    ctx.accounts.operation.last_minted = 0;
    ctx.accounts.operation.cool_down_period = 7200; // default 60 seconds
    Ok(())
}

#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        seeds = [OPERATION_SEED.as_ref()],
        bump,
        payer = signer,
        space = 8 + Operation::LEN,
    )]
    pub operation: Account<'info, Operation>,

    /// CHECK: This is a mint account that will be created by the token program
    #[account(
        init,
        payer = signer,
        mint::decimals = 6,
        mint::authority = mint,
        mint::freeze_authority = mint,
        seeds = [MINT_SEED.as_ref()],
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
        seeds = [TOKEN_ACCOUNT_SEED.as_ref()],
        bump
    )]
    pub token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
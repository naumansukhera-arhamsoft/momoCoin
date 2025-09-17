use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitOracleData<'info> {
    #[account(
        init,
        seeds = [b"oracle_data".as_ref()],
        bump,
        space = 8 + OracleData::INIT_SPACE,
        payer = user,
    )]
    pub oracle_data_account: Account<'info, OracleData>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitOracleData>) -> Result<()> {
    let data = &mut ctx.accounts.oracle_data_account;
    data.latest_pulse = 0;
    data.last_updated = 0;
    data.admin = ctx.accounts.user.key();
    Ok(())
}

use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;

pub use instructions::*;
pub use state::*;

declare_id!("7WS182qgZYwY2wiqZkX5LBvw43fsfkHtp9kKCXtF5kXp");

#[program]
pub mod oracle {
    use super::*;

    pub fn initialize_oracle(ctx: Context<InitOracleData>) -> Result<()> {
        instructions::initialize_oracle::handler(ctx)
    }

    pub fn add_pulse(ctx: Context<OracleAccount>, available_bank_balance: u64) -> Result<String> {
        instructions::add_pulse::handler(ctx, available_bank_balance)
    }
}
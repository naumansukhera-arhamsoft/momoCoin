use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;

pub use instructions::*;
pub use state::*;
pub use errors::*;

declare_id!("2vPErEWHjdGAv5JBYYN72X1UR51sAFJ3LHVHnqaEinJH");

#[program]
pub mod minter {
    use super::*;

    pub fn initialize(ctx: Context<CreateMint>, oracle: Pubkey) -> Result<()> {
        instructions::initialize(ctx, oracle)
    }

    pub fn update_oracle(ctx: Context<UpdateOperation>, oracle: Pubkey) -> Result<()> {
        instructions::update_oracle(ctx, oracle)
    }

    pub fn update_admin(ctx: Context<UpdateOperation>, admin: Pubkey) -> Result<()> {
        instructions::update_admin(ctx, admin)
    }
     pub fn update_status(ctx: Context<UpdateOperation>, status: u8) -> Result<()> {
        instructions::update_status(ctx, status)
    }

    pub fn mint_tokens(ctx: Context<TokenOperations>, amount: u64) -> Result<()> {
        instructions::mint_tokens(ctx, amount)
    }

    pub fn burn_tokens(ctx: Context<TokenOperations>, amount: u64) -> Result<()> {
        instructions::burn_tokens(ctx, amount)
    }

    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        instructions::transfer_tokens(ctx, amount)
    }

    pub fn get_stats_supply(ctx: Context<TokenOperations>) -> Result<u64> {
        instructions::get_stats_supply(ctx)
    }

    pub fn create_metadata(
        ctx: Context<CreateMetadata>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        instructions::create_metadata(ctx, name, symbol, uri)
    }
}
use anchor_lang::prelude::*;

/// Events emitted by the oracle program
#[event]
pub struct PulseRecorded {
    pub pulse_number: u64,
    pub available_balance: u64,
    pub token_supply: u64,
    pub timestamp: i64,
    pub authority: Pubkey,
    pub action_taken: String, // "mint", "burn", or "none"
    pub amount_changed: u64,
}

#[event]
pub struct AdminUpdated {
    pub old_admin: Pubkey,
    pub new_admin: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct OracleInitialized {
    pub admin: Pubkey,
    pub timestamp: i64,
}

/// Utility functions for common operations
pub mod utils {
    use super::*;
    use crate::errors::ErrorCode;
    use crate::constants::constants::{ORACLE_DATA_SEED};
    
    /// Safely calculates the difference for minting/burning
    pub fn calculate_safe_difference(
        available_balance: u64,
        current_supply: u64,
    ) -> Result<(u64, bool)> { // (amount, should_mint)
        if available_balance == current_supply {
            return Ok((0, false));
        }
        
        let (difference, should_mint) = if available_balance > current_supply {
            (available_balance.saturating_sub(current_supply), true)
        } else {
            (current_supply.saturating_sub(available_balance), false)
        };
        
        Ok((difference, should_mint))
    }
    
    /// Validates that the mint has the correct authority
      /// Validates that the mint has the correct authority
    pub fn verify_mint_authority(
        mint: &anchor_spl::token::Mint,
        expected_authority: &Pubkey,
    ) -> Result<()> {
        match mint.mint_authority {
            anchor_spl::token::COption::Some(authority) => {
                require_keys_eq!(
                    authority,
                    *expected_authority,
                    ErrorCode::InvalidMintAuthority
                );
            }
            anchor_spl::token::COption::None => return Err(ErrorCode::InvalidMintAuthority.into()),
        }
        Ok(())
    }
    /// Creates standardized PDA seeds
    /// Creates standardized PDA seeds
    pub fn create_oracle_data_seeds(bump: u8) -> [Vec<u8>; 2] {
        let bump_arr = vec![bump];
        [ORACLE_DATA_SEED.to_vec(), bump_arr]
    }
    
    /// Validates timestamp is not in the future
    pub fn validate_timestamp(timestamp: i64) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        require!(timestamp <= current_time, ErrorCode::InvalidTimestamp);
        Ok(())
    }
}
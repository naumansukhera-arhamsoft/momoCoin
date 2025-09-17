use anchor_lang::prelude::*;

#[account]
pub struct OraclePulse {
    /// Available bank balance, stored in base units (8 decimals).
    pub available_bank_balance: u64, // 8 bytes
    pub pulse: u64, // 8 bytes
    pub timestamp: i64, // 8 bytes
                    // Total data: 24 bytes + 8 bytes discriminator = 32 bytes
}


impl Space for OraclePulse {
    const INIT_SPACE: usize = 8 + // discriminator
     8 + // available_bank_balance
        8 + // pulse
        8+ // timestamp
        8;// extra_space  ;
}

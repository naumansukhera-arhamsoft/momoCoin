use anchor_lang::prelude::*;

#[account]
pub struct OracleData {
    pub latest_pulse: u64,           // 8 bytes
    pub admin: Pubkey,               // 32 bytes
    pub last_updated: i64,           // 8 bytes

                                     // Total data: 48 bytes + 8 bytes discriminator = 56 bytes
}

impl Space for OracleData {
    const INIT_SPACE: usize = 8+// discriminator
    8+ // latest_pulse
    32+ // admin
    8+ // last_updated
    8; // extra_space
}

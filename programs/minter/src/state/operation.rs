use anchor_lang::prelude::*;

#[account]
pub struct Operation {
    pub admin: Pubkey,    // 32 bytes
    pub oracle: Pubkey,   // 32 bytes
    pub status: u8,       // 1 byte - 0 = paused,1 = active,3 = Freezed
    pub last_minted: i64, // 8 bytes
    pub cool_down_period: u64, // 8 bytes

}

impl Operation {
    // Total size of fields (excluding discriminator):
    // 32 (admin) + 32 (oracle) + 1 (status) + 8 (last_minted) + 8 (cool_down_period) = 81
    // Add padding for future-proofing
    pub const LEN: usize = 32 + 32 + 1 + 8 + 8 + 8; // 89 bytes
}
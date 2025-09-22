use anchor_lang::prelude::*;

#[account]
pub struct Operation {
    pub admin: Pubkey,    // 32 bytes
    pub oracle: Pubkey,   // 32 bytes
    pub status: u8,       // 1 byte - 0 = paused,1 = active,3= Freezed
}

impl Operation {
    pub const LEN: usize = 8 + // discriminator
        32 + // admin
        32 + // oracle
        10+// status
        8; //extra space;   
}
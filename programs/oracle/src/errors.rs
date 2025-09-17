use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Integer overflowed")]
    Overflow,
    #[msg("Unauthorized user attempted to modify oracle data")]
    UnAuthorizedUser,
}
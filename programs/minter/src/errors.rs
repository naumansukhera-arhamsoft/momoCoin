use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized oracle program")]
    UnauthorizedOracle,
    #[msg("Unauthorized Admin user")]
    UnauthorizedAdminUser,
    #[msg("Metadata PDA is invalid")]
    InvalidMetadataPDA,
    #[msg("Status is invalid")]
    InvalidStatus,
    #[msg("Operation is paused")]
    OperationPaused,
    #[msg("Operation is freezed")]
    OperationFreezed,
    #[msg("Cool down period has not elapsed since last minting")]
    CoolDownPeriodNotElapsed,
}
/// Program constants and configuration
pub mod constants {
    /// Maximum pulse count to prevent overflow attacks
    pub const MAX_PULSE_COUNT: u64 = u64::MAX - 1000; // Leave buffer for safety
    
    /// Minimum time between pulses in seconds (prevents spam)
    pub const MIN_PULSE_INTERVAL: i64 = 30; // 30 seconds
    
    /// Maximum allowed balance difference percentage (10%)
    pub const MAX_BALANCE_DEVIATION_BPS: u64 = 1000; // 10% in basis points
    
    /// Seeds for PDA derivation
    pub const ORACLE_DATA_SEED: &[u8] = b"oracle_data";
    pub const ORACLE_PULSE_SEED: &[u8] = b"oracle_pulse";
    
    /// Account space constants
    pub const ORACLE_DATA_SPACE: usize = 8 + 8 + 32 + 8 + 8; // discriminator + latest_pulse + admin + last_updated + total_pulses
    pub const ORACLE_PULSE_SPACE: usize = 8 + 8 + 8 + 8 + 32; // discriminator + available_bank_balance + pulse + timestamp + authority
}
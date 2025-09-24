/// Program constants and configuration
pub mod constants {
    /// Seeds for PDA derivation
    pub const OPERATION_SEED: &[u8] = b"operation";
    pub const MINT_SEED: &[u8] = b"mint";
    pub const TOKEN_ACCOUNT_SEED: &[u8] = b"token";
    //Oracle constants
    pub const ORACLE_DATA_SEED: &[u8] = b"oracle_data";
    pub const ORACLE_PULSE_SEED: &[u8] = b"oracle";
}

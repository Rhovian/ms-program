use anchor_lang::prelude::*;

#[error_code]
pub enum MadscapeError {
    #[msg("Invalid mint")]
    InvalidFeeMint,
    #[msg("Mint already approved")]
    FeeMintAlreadyApproved,
    #[msg("Mint not approved")]
    FeeMintNotApproved,
    #[msg("Invalid token account ammount")]
    InvalidTokenAccountAmount,
    #[msg("Escrow is initialized")]
    EscrowInitialized,
    #[msg("Escrow is not initialized")]
    EscrowNotInitialized,
    #[msg("Escrow is not activated")]
    EscrowNotActivated,
    #[msg("Escrow is active")]
    EscrowIsActive,
    #[msg("Invalid user b mint")]
    InvalidUserBMint,
    #[msg("Invalid Winner")]
    InvalidWinner,
    #[msg("Insufficient Funds")]
    InsufficientFunds,
    #[msg("Escrow is not native sol")]
    EscrowNotNativeSol,
    #[msg("Escrow is native sol")]
    EscrowIsNativeSol,
    #[msg("Escrow is not active")]
    EscrowNotActive,
    #[msg("Escrow is active")]
    EscrowActive,
    #[msg("Numeric overflow")]
    NumericOverflow,
    #[msg("Invalid match type")]
    InvalidMatchType,
    #[msg("User a and user b are the same")]
    UserAAndUserBAreTheSame,
    #[msg("Unimplemented")]
    Unimplemented,
    #[msg("Fee Calculation Failure")]
    FeeCalculationFailure,
    #[msg("Invalid Loser Mint")]
    InvalidLoserMint,
    #[msg("Invalid User B")]
    InvalidUserB,
}

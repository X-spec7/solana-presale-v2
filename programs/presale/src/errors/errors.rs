use anchor_lang::prelude::*;

#[error_code]
pub enum PresaleError {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("Not allowed")]
    NotAllowed,
    #[msg("Math operation overflow")]
    MathOverflow,
    #[msg("Already marked")]
    AlreadyMarked,
    #[msg("Presale not started yet")]
    PresaleNotStarted,
    #[msg("Presale already ended")]
    PresaleEnded,
    #[msg("Token amount mismatch")]
    TokenAmountMismatch,
    #[msg("Insufficient Tokens")]
    InsufficientFund,
    #[msg("Presale not ended yet")]
    PresaleNotEnded,
    #[msg("Presale already ended")]
    HardCapped,
    #[msg("No funds to refund")]
    NoFundsToRefund,
    #[msg("Presale is still ongoing")]
    PresaleStillOngoing,
    #[msg("Softcap has been reached, refunds are not possible")]
    SoftcapReached,
    #[msg("Error in token amount calculation")]
    CalculationError,
    #[msg("Purchase would exceed maximum tokens per address")]
    ExceedsMaxTokenPerAddress,
    #[msg("No tokens to claim")]
    NoTokensToClaim,
    #[msg("Insufficient presale tokens")]
    InsufficientPresaleTokens,
}
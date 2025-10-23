use anchor_lang::error_code;

#[error_code]
pub enum CountError{
    #[msg("Value out of bounds")]
    CountOverflowUnderflow,
}

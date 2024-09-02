use anchor_lang::error_code;

#[error_code]
pub enum Errors {
    #[msg("You are not authorized to perform this action")]
    Forbidden,

    #[msg("Invalid token")]
    InvalidToken,

    #[msg("Incorrect amount")]
    IncorrectAmount,

    #[msg("Amount per wallet must be more than 0")]
    IncorrectAmountPerWallet,
}
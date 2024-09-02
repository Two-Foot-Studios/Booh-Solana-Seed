use anchor_lang::prelude::*;

use crate::constants::*;
use std::str::FromStr;

pub fn validate_init(
    admin_key: &Pubkey,
    token_key: &Pubkey,
    amount: u64
) -> Result<()> {
    require!(amount > 0, Errors::IncorrectAmount);
    require!(*admin_key == Pubkey::from_str(ADMIN_KEY).unwrap(), Errors::Forbidden);
    require!(*token_key == Pubkey::from_str(TOKEN_MINT).unwrap(), Errors::InvalidToken);

    let amount_per_wallet = amount / WALLETS_COUNT;
    require!(amount_per_wallet > 0, Errors::IncorrectAmountPerWallet);

    Ok(())
}

pub fn validate_mint(token_key: &Pubkey, user_key: &Pubkey) -> Result<()> {
    require!(*token_key == Pubkey::from_str(TOKEN_MINT).unwrap(), Errors::InvalidToken);
    require!(is_valid_user_pubkey(user_key), Errors::Forbidden);

    Ok(())
}

fn is_valid_user_pubkey(user_key: &Pubkey) -> bool {
    USERS_KEYS.iter().any(|&key_str| Pubkey::from_str(key_str).unwrap() == *user_key)
}

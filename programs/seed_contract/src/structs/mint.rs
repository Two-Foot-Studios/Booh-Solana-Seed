use anchor_lang::Accounts;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use crate::structs::{MintStat, UserMintStat};

#[derive(Accounts)]
pub struct Mint<'info> {
    pub mint_of_token: Account<'info, anchor_spl::token::Mint>,

    #[account(
        mut,
        constraint = user_token_ata.owner == user.key(),
        constraint = user_token_ata.mint == mint_of_token.key()
    )]
    pub user_token_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + 8
    )]
    pub user_mint_stat: Account<'info, UserMintStat>,

    #[account(
        mut,
        seeds = [b"mint_stat"],
        bump,
    )]
    pub mint_stat: Account<'info, MintStat>,

    #[account(
        mut,
        seeds = [b"mint_bank"],
        bump,
        token::mint = mint_of_token,
        token::authority = mint_bank
    )]
    pub mint_bank: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
use anchor_lang::Accounts;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::structs::mint_stat::MintStat;

#[derive(Accounts)]
pub struct InitMain<'info> {
    pub mint_of_token: Account<'info, Mint>,

    #[account(
        mut,
        constraint = admin_token_ata.owner == admin.key(),
        constraint = admin_token_ata.mint == mint_of_token.key()
    )]
    pub admin_token_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        seeds = [b"mint_stat"],
        bump,
        payer = admin,
        space = 8 + 8 + 8 + 8 + 8
    )]
    pub mint_stat: Account<'info, MintStat>,

    #[account(
        init,
        seeds = [b"mint_bank"],
        bump,
        payer = admin,
        token::mint = mint_of_token,
        token::authority = mint_bank
    )]
    pub mint_bank: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
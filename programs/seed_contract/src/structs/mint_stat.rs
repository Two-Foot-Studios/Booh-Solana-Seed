use anchor_lang::prelude::*;

#[account]
pub struct MintStat {
    pub start: i64,
    pub end: i64,
    pub amount: u64,
    pub amount_per_account: u64
}
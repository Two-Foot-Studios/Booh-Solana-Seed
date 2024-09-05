use structs::*;
use functions::*;
mod structs;
mod constants;
mod functions;

use anchor_lang::prelude::*;
use anchor_spl::token::Transfer;


declare_id!("5a8S3GcbPbutDnCLMiwQ1tZLCLyermnKQpiydfs8DMw7");

#[program]
pub mod seed_contract {
    use anchor_spl::token::TransferChecked;

    use crate::constants::{SECONDS_IN_YEAR, WALLETS_COUNT};
    use super::*;

    // Admin function for loading tokens to smart contract
    // Must be call as first action on contract
    // Without it all other function can't work
    pub fn initialize(ctx: Context<InitMain>, amount: u64) -> Result<()> {
        validate_init(
            &ctx.accounts.admin.key(),
            &ctx.accounts.mint_of_token.key(),
            amount
        )?;

        let transfer_instructions = Transfer {
            from: ctx.accounts.admin_token_ata.to_account_info(),
            to: ctx.accounts.mint_bank.to_account_info(),
            authority: ctx.accounts.admin.to_account_info()
        };

        let binding = [ctx.bumps.mint_bank];
        let seed = vec![
            b"mint_bank",
            binding.as_slice()
        ];
        let outer = vec![seed.as_slice()];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_instructions,
            outer.as_slice()
        );

        anchor_spl::token::transfer(cpi_ctx, amount)?;

        ctx.accounts.mint_stat.amount = amount;
        ctx.accounts.mint_stat.start = Clock::get()?.unix_timestamp;
        ctx.accounts.mint_stat.end = ctx.accounts.mint_stat.start + SECONDS_IN_YEAR;
        ctx.accounts.mint_stat.amount_per_account = amount / WALLETS_COUNT;

        Ok(())
    }

    // User function for minting and withdrawing tokens at the same time
    pub fn mint(ctx: Context<Mint>) -> Result<()> {
        validate_mint(&ctx.accounts.mint_of_token.key(), &ctx.accounts.user.key())?;

        if ctx.accounts.user_mint_stat.last_reward >= ctx.accounts.mint_stat.end {
            return Ok(());
        }

        let amount = calculate_mint_amount(
            ctx.accounts.user_mint_stat.last_reward,
            &ctx.accounts.mint_stat
        );

        if amount == 0 {
            return Ok(());
        }

        let transfer_instruction = TransferChecked {
            from: ctx.accounts.mint_bank.to_account_info(),
            to: ctx.accounts.user_token_ata.to_account_info(),
            authority: ctx.accounts.mint_bank.to_account_info(),
            mint: ctx.accounts.mint_of_token.to_account_info()
        };

        let binding = [ctx.bumps.mint_bank];
        let seed = vec![
            b"mint_bank",
            binding.as_slice()
        ];
        let outer = vec![seed.as_slice()];


        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
            outer.as_slice()
        );

        anchor_spl::token::transfer_checked(
            cpi_ctx,
            amount,
            ctx.accounts.mint_of_token.decimals
        )?;

        ctx.accounts.user_mint_stat.last_reward = Clock::get().unwrap().unix_timestamp;

        Ok(())
    }
}


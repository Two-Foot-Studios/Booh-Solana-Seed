use anchor_lang::prelude::*;
use crate::constants::SECONDS_IN_YEAR;

use crate::structs::MintStat;

pub fn calculate_mint_amount(last_reward_unix: i64, stat: &MintStat) -> u64 {
    if last_reward_unix >= stat.end {
        return 0;
    }

    let current_time: i64 = Clock::get().unwrap().unix_timestamp;

    if last_reward_unix == 0 {
        return if current_time >= stat.end {
            stat.amount_per_account
        } else {
            calculate_amount(stat.amount_per_account, current_time - stat.start)
        }
    }

    return if current_time >= stat.end {
        calculate_amount(stat.amount_per_account, stat.end - last_reward_unix)
    } else {
        calculate_amount(stat.amount_per_account, current_time - last_reward_unix)
    }
}

fn calculate_amount(full_amount: u64, time_passed: i64) -> u64 {
    let part_of_full: f64 = time_passed as f64 / SECONDS_IN_YEAR as f64;
    (full_amount as f64 * part_of_full) as u64
}

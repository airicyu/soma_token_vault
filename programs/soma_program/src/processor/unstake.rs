use crate::constant::{ LOCK_PERIOD_SLOTS, SEED_PREFIX_USER_STAKE_INFO, SEED_STAKE_POOL };
use crate::state::stake_pool::StakePool;
use crate::state::user_stake_info::UserStakeInfo;
use crate::error::ProtocolProgramError;
use anchor_lang::prelude::*;

/// Unstake token
#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(seeds = [SEED_STAKE_POOL.as_ref()], bump)]
    pub stake_pool: Box<Account<'info, StakePool>>,

    #[account(mut, seeds = [SEED_PREFIX_USER_STAKE_INFO, payer.key().as_ref()], bump)]
    pub user_stake_info: Account<'info, UserStakeInfo>,

    pub system_program: Program<'info, System>,
}

pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
    let user_stake_info = &mut ctx.accounts.user_stake_info;

    require!(
        user_stake_info.is_initialized &&
            (user_stake_info.token_balance > 0 || user_stake_info.reward_balance > 0),
        ProtocolProgramError::NoBalanceToUnstake
    );

    let clock = Clock::get().unwrap();
    user_stake_info
        .unstake(
            amount,
            ctx.accounts.stake_pool.stake_base_rate,
            clock.slot,
            clock.slot + LOCK_PERIOD_SLOTS
        )
        .unwrap();

    msg!("after unstake:");
    user_stake_info.println();

    Ok(())
}

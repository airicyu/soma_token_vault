use anchor_lang::prelude::*;

use crate::constant::{ SEED_OWNER_CONFIG, SEED_STAKE_POOL };
use crate::error::ProtocolProgramError;
use crate::state::owner_config::*;
use crate::state::stake_pool::StakePool;

/// Change stake rate
#[derive(Accounts)]
pub struct ChangeStakeRateContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, seeds = [SEED_OWNER_CONFIG.as_ref()], bump,
    constraint = owner_config.is_owner(&payer.key()) @ ProtocolProgramError::RequireOwner)]
    pub owner_config: Account<'info, OwnerConfig>,

    #[account(mut,
        seeds = [SEED_STAKE_POOL.as_ref()],
        bump
    )]
    pub stake_pool: Box<Account<'info, StakePool>>,
}

pub fn change_stake_rate(ctx: Context<ChangeStakeRateContext>, stake_base_rate: u32) -> Result<()> {
    let stake_pool = &mut ctx.accounts.stake_pool;
    stake_pool.stake_base_rate = stake_base_rate;

    Ok(())
}

use anchor_lang::prelude::*;

use crate::constant::SEED_OWNER_CONFIG;
use crate::error::ProtocolProgramError;
use crate::state::owner_config::*;

/// Change owner
#[derive(Accounts)]
pub struct ChangeOwnerContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, seeds = [SEED_OWNER_CONFIG.as_ref()], bump,
    constraint = owner_config.is_owner(&payer.key()) @ ProtocolProgramError::RequireOwner)]
    pub owner_config: Account<'info, OwnerConfig>,
}

pub fn change_owner(ctx: Context<ChangeOwnerContext>, owner: Pubkey) -> Result<()> {
    let owner_config = &mut ctx.accounts.owner_config;
    owner_config.owner = owner;

    Ok(())
}

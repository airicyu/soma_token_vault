use anchor_lang::prelude::*;

use crate::constant::SEED_OWNER_CONFIG;
use crate::error::ProtocolProgramError;
use crate::state::owner_config::*;

/// Change owner
#[derive(Accounts)]
pub struct ChangeOwner<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(seeds = [SEED_OWNER_CONFIG.as_ref()], bump)]
    pub owner_config: Account<'info, OwnerConfig>,
}

pub fn change_owner(ctx: Context<ChangeOwner>, owner: Pubkey) -> Result<()> {
    require!(
        &ctx.accounts.owner_config.is_owner(&ctx.accounts.payer.key()),
        ProtocolProgramError::RequireOwner
    );

    let owner_config = &mut ctx.accounts.owner_config;
    owner_config.owner = owner;

    Ok(())
}

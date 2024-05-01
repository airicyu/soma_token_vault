use anchor_lang::prelude::*;

use crate::constant::{ SEED_OWNER_CONFIG, SEED_RELAYER_CONFIG };
use crate::error::ProtocolProgramError;
use crate::state::owner_config::*;
use crate::state::relayer_config::RelayerConfig;

/// Change relayer
#[derive(Accounts)]
pub struct ChangeRelayer<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(seeds = [SEED_OWNER_CONFIG.as_ref()], bump)]
    pub owner_config: Account<'info, OwnerConfig>,

    #[account(seeds = [SEED_RELAYER_CONFIG.as_ref()], bump)]
    pub relayer_config: Account<'info, RelayerConfig>,
}

pub fn change_relayer(ctx: Context<ChangeRelayer>, relayer: Pubkey) -> Result<()> {
    require!(
        &ctx.accounts.owner_config.is_owner(&ctx.accounts.payer.key()),
        ProtocolProgramError::RequireOwner
    );

    let relayer_config = &mut ctx.accounts.relayer_config;
    relayer_config.relayer = relayer;

    Ok(())
}

use crate::constant::{
    SEED_MINTER,
    SEED_OWNER_CONFIG,
    SEED_RELAYER_CONFIG,
    SEED_STAKE_POOL,
    SEED_VAULT,
};
use crate::state::stake_pool::StakePool;
use crate::state::vault::Vault;
use crate::state::{ owner_config::*, relayer_config::* };
use anchor_lang::prelude::*;

/// Initialize program
///
/// Must be run once after program deployed.
#[derive(Accounts)]
#[instruction(data: Box<InitializeProgramInstructionData>)]
pub struct InitializeProgram<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        seeds = [SEED_OWNER_CONFIG.as_ref()],
        bump,
        space = 8 + OwnerConfig::INIT_SPACE
    )]
    pub owner_config: Box<Account<'info, OwnerConfig>>,

    #[account(
        init,
        payer = payer,
        seeds = [SEED_RELAYER_CONFIG.as_ref()],
        bump,
        space = 8 + RelayerConfig::INIT_SPACE
    )]
    pub relayer_config: Box<Account<'info, RelayerConfig>>,

    // #[account(init, payer = payer, seeds = [SEED_PREFIX_MINTER.as_ref()], bump, space = 8)]
    #[account(seeds = [SEED_MINTER.as_ref()], bump)]
    pub minter: AccountInfo<'info>,

    #[account(
        init,
        payer = payer,
        seeds = [SEED_VAULT.as_ref()],
        bump,
        space = 8 + Vault::INIT_SPACE
    )]
    pub vault: Box<Account<'info, Vault>>,

    #[account(
        init,
        payer = payer,
        seeds = [SEED_STAKE_POOL.as_ref()],
        bump,
        space = 8 + StakePool::INIT_SPACE
    )]
    pub stake_pool: Box<Account<'info, StakePool>>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct InitializeProgramInstructionData {
    pub owner: Pubkey,
    pub relayer: Pubkey,
}

pub fn initialize_program(
    ctx: Context<InitializeProgram>,
    data: Box<InitializeProgramInstructionData>
) -> Result<()> {
    // msg!("data {:?}", data);
    let owner_config = &mut ctx.accounts.owner_config;
    owner_config.owner = data.owner; //ctx.accounts.payer.key();

    let relayer_config: &mut Account<'_, RelayerConfig> = &mut ctx.accounts.relayer_config;
    relayer_config.relayer = data.relayer; //ctx.accounts.payer.key();

    msg!("Owner: {:?}", owner_config.owner);
    msg!("Relayer: {:?}", relayer_config.relayer);
    msg!("payer: {:?}", ctx.accounts.payer.key());

    let minter = &mut ctx.accounts.minter;
    msg!("minter: {:?}", minter.key());

    let vault = &mut ctx.accounts.vault;
    vault.balance = 0u64;

    msg!("vault: {:?}", vault.key());

    let stake_pool = &mut ctx.accounts.stake_pool;
    stake_pool.balance = 0u64;
    stake_pool.stake_base_rate = 1000u32; // 10.00%

    msg!("stake_pool: {:?}", stake_pool.key());

    Ok(())
}

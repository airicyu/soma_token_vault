use crate::constant::{
    SEED_MINT,
    SEED_MINTER,
    SEED_OWNER_CONFIG,
    SEED_STAKE_POOL,
    SEED_STAKE_POOL_TOKEN_ACCOUNT,
    SEED_TOKEN_CONFIG,
    SEED_VAULT,
    SEED_VAULT_TOKEN_ACCOUNT,
};
use crate::error::ProtocolProgramError;
use crate::state::stake_pool::StakePool;
use crate::state::token_config::TokenConfig;
use crate::state::vault::Vault;
use crate::state::owner_config::*;
use anchor_lang::prelude::*;
use anchor_spl::token_2022::Token2022;
use anchor_spl::token_interface::{ Mint, TokenAccount };

/// Initialize token
///
/// Must be run once after program deployed. And should run after "InitializeProgram".
#[derive(Accounts)]
#[instruction(data: Box<InitializeTokenInstructionData>)]
pub struct InitializeTokenContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, seeds = [SEED_OWNER_CONFIG.as_ref()], bump,
    constraint = owner_config.is_owner(&payer.key()) @ ProtocolProgramError::RequireOwner)]
    pub owner_config: Account<'info, OwnerConfig>,

    #[account(
        init,
        payer = payer,
        seeds = [SEED_TOKEN_CONFIG.as_ref()],
        bump,
        space = 8 + TokenConfig::INIT_SPACE
    )]
    pub token_config: Box<Account<'info, TokenConfig>>,

    // #[account(init, payer = payer, seeds = [SEED_PREFIX_MINTER.as_ref()], bump, space = 8)]
    #[account(seeds = [SEED_MINTER.as_ref()], bump)]
    pub minter: AccountInfo<'info>,

    #[account(
        init,
        payer = payer,
        mint::decimals = data.token_decimals.clone(),
        mint::authority = minter,
        seeds = [SEED_MINT],
        bump
    )]
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(seeds = [SEED_VAULT.as_ref()], bump)]
    pub vault: Box<Account<'info, Vault>>,

    #[account(
        init,
        payer = payer,
        seeds = [SEED_VAULT_TOKEN_ACCOUNT],
        bump,
        token::mint = mint,
        token::authority = vault
    )]
    pub vault_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(seeds = [SEED_STAKE_POOL.as_ref()], bump)]
    pub stake_pool: Box<Account<'info, StakePool>>,

    #[account(
        init,
        payer = payer,
        seeds = [SEED_STAKE_POOL_TOKEN_ACCOUNT],
        bump,
        token::mint = mint,
        token::authority = stake_pool
    )]
    pub stake_pool_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct InitializeTokenInstructionData {
    pub token_decimals: u8,
}

pub fn initialize_token(
    ctx: Context<InitializeTokenContext>,
    data: Box<InitializeTokenInstructionData>
) -> Result<()> {
    let token_config = &mut ctx.accounts.token_config;
    token_config.mint = ctx.accounts.mint.key();

    msg!("mint: {:?}", ctx.accounts.token_config.key());
    msg!("token_decimals: {}", data.token_decimals);

    let minter = &mut ctx.accounts.minter;
    msg!("minter: {:?}", minter.key());

    let vault = &mut ctx.accounts.vault;
    vault.balance = 0u64;

    msg!("vault_token_account: {}", ctx.accounts.vault_token_account.key());
    msg!("stake_pool_token_account: {}", ctx.accounts.stake_pool_token_account.key());

    Ok(())
}

use crate::constant::{
    SEED_MINTER,
    SEED_PREFIX_USER_STAKE_INFO,
    SEED_STAKE_POOL,
    SEED_STAKE_POOL_TOKEN_ACCOUNT,
    SEED_TOKEN_CONFIG,
};
use crate::state::stake_pool::StakePool;
use crate::state::token_config::TokenConfig;
use crate::state::user_stake_info::UserStakeInfo;
use anchor_lang::prelude::*;
use anchor_spl::token_2022::{ transfer_checked, Token2022, TransferChecked };
use anchor_spl::token_interface::{ Mint, TokenAccount };

/// Stake token to pool
#[derive(Accounts)]
pub struct StakeContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(seeds = [SEED_TOKEN_CONFIG.as_ref()], bump)]
    pub token_config: Box<Account<'info, TokenConfig>>,

    #[account(seeds = [SEED_MINTER.as_ref()], bump)]
    pub minter: AccountInfo<'info>,

    #[account(address = token_config.mint)]
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut, seeds = [SEED_STAKE_POOL.as_ref()], bump)]
    pub stake_pool: Box<Account<'info, StakePool>>,

    #[account(mut, seeds = [SEED_STAKE_POOL_TOKEN_ACCOUNT], bump,
        token::mint = mint,
        token::authority = stake_pool
    )]
    pub stake_pool_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = payer,
        seeds = [SEED_PREFIX_USER_STAKE_INFO, payer.key().as_ref()],
        bump,
        space = 8 + UserStakeInfo::INIT_SPACE
    )]
    pub user_stake_info: Account<'info, UserStakeInfo>,

    #[account(mut, 
        associated_token::mint = mint,
        associated_token::authority = payer,
        associated_token::token_program = token_program
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn stake(ctx: Context<StakeContext>, amount: u64) -> Result<()> {
    let clock = Clock::get().unwrap();

    // transfer user token to stake pool
    {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                mint: ctx.accounts.mint.to_account_info(),
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.stake_pool_token_account.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            }
        );
        transfer_checked(cpi_ctx, amount, ctx.accounts.mint.decimals).unwrap();
    }

    // update stake pool balance
    let stake_pool: &mut Box<Account<'_, StakePool>> = &mut ctx.accounts.stake_pool;
    stake_pool.balance = stake_pool.balance.checked_add(amount).unwrap();

    msg!("User staked {} token to pool, pool after balance {}", amount, stake_pool.balance);

    // update user stake info
    let user_stake_info = &mut ctx.accounts.user_stake_info;
    let is_init_stake = !user_stake_info.is_initialized;

    if is_init_stake {
        msg!("User init stake");
        user_stake_info.initialize();
        user_stake_info.initial_stake(amount, clock.slot).unwrap();
    } else {
        msg!("User stake");
        user_stake_info.stake(amount, stake_pool.stake_base_rate, clock.slot).unwrap();
    }
    user_stake_info.println();

    Ok(())
}

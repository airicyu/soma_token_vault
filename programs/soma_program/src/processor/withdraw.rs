use crate::constant::{
    SEED_MINTER,
    SEED_PREFIX_USER_STAKE_INFO,
    SEED_STAKE_POOL,
    SEED_STAKE_POOL_TOKEN_ACCOUNT,
    SEED_TOKEN_CONFIG,
    SEED_VAULT,
    SEED_VAULT_TOKEN_ACCOUNT,
};
use crate::error::ProtocolProgramError;
use crate::state::stake_pool::StakePool;
use crate::state::token_config::TokenConfig;
use crate::state::user_stake_info::UserStakeInfo;
use crate::state::vault::Vault;
use anchor_lang::prelude::*;
use anchor_spl::token_2022::{ transfer_checked, Token2022, TransferChecked };
use anchor_spl::token_interface::{ Mint, TokenAccount };

/// Withdraw token from unstaked balance
#[derive(Accounts)]
pub struct WithdrawContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(seeds = [SEED_TOKEN_CONFIG.as_ref()], bump)]
    pub token_config: Box<Account<'info, TokenConfig>>,

    #[account(mut, seeds = [SEED_MINTER.as_ref()], bump)]
    pub minter: AccountInfo<'info>,

    #[account(mut, address = token_config.mint)]
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut, seeds = [SEED_VAULT.as_ref()], bump)]
    pub vault: Box<Account<'info, Vault>>,

    #[account(mut, 
        seeds = [SEED_VAULT_TOKEN_ACCOUNT], bump,
        token::mint = token_config.mint,
        token::authority = vault,
        token::token_program = token_program
    )]
    pub vault_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut, seeds = [SEED_STAKE_POOL.as_ref()], bump)]
    pub stake_pool: Box<Account<'info, StakePool>>,

    #[account(mut, seeds = [SEED_STAKE_POOL_TOKEN_ACCOUNT], bump,
        token::mint = mint,
        token::authority = stake_pool
    )]
    pub stake_pool_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
            mut,
        seeds = [SEED_PREFIX_USER_STAKE_INFO, payer.key().as_ref()],
        bump
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

pub fn withdraw(ctx: Context<WithdrawContext>) -> Result<()> {
    let clock = Clock::get().unwrap();

    let user_stake_info = &mut ctx.accounts.user_stake_info;

    // check have balance to withdraw
    require!(
        user_stake_info.is_initialized &&
            (user_stake_info.lock_balance > 0 || user_stake_info.lock_reward_balance > 0),
        ProtocolProgramError::NoBalanceToWithdraw
    );

    // check lock until slot
    require!(
        clock.slot > user_stake_info.lock_until_slot,
        ProtocolProgramError::CannotWithdrawLockingAsset
    );

    let pool_be_tran_out_amount = user_stake_info.lock_balance;
    let vault_be_tran_out_reward = user_stake_info.lock_reward_balance;

    if pool_be_tran_out_amount > 0 {
        // transfer from stake pool to user token
        {
            let bump = ctx.bumps.stake_pool;
            let seeds = &[SEED_STAKE_POOL.as_ref(), &[bump]];
            let signer: &[&[&[u8]]] = &[&seeds[..]];

            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    mint: ctx.accounts.mint.to_account_info(),
                    from: ctx.accounts.stake_pool_token_account.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.stake_pool.to_account_info(),
                },
                signer
            );
            transfer_checked(cpi_ctx, pool_be_tran_out_amount, ctx.accounts.mint.decimals).unwrap();
        }
    }

    if vault_be_tran_out_reward > 0 {
        // transfer from vault pool to user token
        {
            let bump = ctx.bumps.vault;
            let seeds = &[SEED_VAULT.as_ref(), &[bump]];
            let signer: &[&[&[u8]]] = &[&seeds[..]];

            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    mint: ctx.accounts.mint.to_account_info(),
                    from: ctx.accounts.vault_token_account.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.vault.to_account_info(),
                },
                signer
            );
            transfer_checked(
                cpi_ctx,
                vault_be_tran_out_reward,
                ctx.accounts.mint.decimals
            ).unwrap();
        }
    }

    // update stake pool balance
    let stake_pool: &mut Box<Account<'_, StakePool>> = &mut ctx.accounts.stake_pool;
    msg!("Tran out {} token from pool", pool_be_tran_out_amount);
    stake_pool.balance = stake_pool.balance.checked_sub(pool_be_tran_out_amount).unwrap();
    msg!("pool after balance {}", stake_pool.balance);

    // update vault balance
    let vault = &mut ctx.accounts.vault;
    msg!("Tran out {} reward token from vault", vault_be_tran_out_reward);
    vault.balance = vault.balance.checked_sub(vault_be_tran_out_reward).unwrap();
    msg!("vault after balance {}", vault.balance);

    // update user stake info
    let user_stake_info = &mut ctx.accounts.user_stake_info;
    msg!(
        "Tran {} token to user with rewards {}, total {}",
        pool_be_tran_out_amount,
        vault_be_tran_out_reward,
        pool_be_tran_out_amount.checked_add(vault_be_tran_out_reward).unwrap()
    );
    user_stake_info.withdraw_from_unstaked(clock.slot).unwrap();

    msg!("After withdraw:");
    user_stake_info.println();

    Ok(())
}

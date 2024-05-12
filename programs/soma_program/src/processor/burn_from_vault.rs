use crate::constant::{
    SEED_MINTER,
    SEED_RELAYER_CONFIG,
    SEED_TOKEN_CONFIG,
    SEED_VAULT,
    SEED_VAULT_TOKEN_ACCOUNT,
};
use crate::error::ProtocolProgramError;
use crate::state::relayer_config::*;
use crate::state::token_config::TokenConfig;
use crate::state::vault::Vault;
use anchor_lang::prelude::*;
use anchor_spl::token_2022::{ burn, Burn, Token2022 };
use anchor_spl::token_interface::{ Mint, TokenAccount };

/// Burn token from vault
#[derive(Accounts)]
pub struct BurnFromVaultContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [SEED_RELAYER_CONFIG.as_ref()],
        bump,
        constraint = relayer_config.is_relayer(&payer.key()) @ ProtocolProgramError::RequireRelayer
    )]
    pub relayer_config: Box<Account<'info, RelayerConfig>>,

    #[account(seeds = [SEED_TOKEN_CONFIG.as_ref()], bump)]
    pub token_config: Box<Account<'info, TokenConfig>>,

    #[account(seeds = [SEED_MINTER.as_ref()], bump)]
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

    pub token_program: Program<'info, Token2022>,

    pub system_program: Program<'info, System>,
}

pub fn burn_from_vault(ctx: Context<BurnFromVaultContext>, amount: u64) -> Result<()> {
    // invoke CPI: Burn vault tokens
    let bump = ctx.bumps.vault;
    let seeds = &[SEED_VAULT.as_ref(), &[bump]];
    let signer: &[&[&[u8]]] = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        },
        signer
    );
    burn(cpi_ctx, amount).unwrap();

    let vault = &mut ctx.accounts.vault;
    vault.balance = vault.balance.checked_sub(amount).unwrap();
    msg!("Burn {} token from vault", amount);

    Ok(())
}

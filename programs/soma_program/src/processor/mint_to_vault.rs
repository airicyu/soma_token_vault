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
use anchor_spl::token_2022::{ mint_to, MintTo, Token2022 };
use anchor_spl::token_interface::{ Mint, TokenAccount };

/// Mint token to vault
#[derive(Accounts)]
pub struct MintToVaultContext<'info> {
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

pub fn mint_to_vault(ctx: Context<MintToVaultContext>, amount: u64) -> Result<()> {
    let bump = ctx.bumps.minter;
    let seeds = &[SEED_MINTER.as_ref(), &[bump]];
    let signer: &[&[&[u8]]] = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.minter.to_account_info(),
        },
        signer
    );
    mint_to(cpi_ctx, amount).unwrap();

    let vault = &mut ctx.accounts.vault;
    vault.balance = vault.balance.checked_add(amount).unwrap();
    msg!("Mint {} token to vault", amount);

    Ok(())
}

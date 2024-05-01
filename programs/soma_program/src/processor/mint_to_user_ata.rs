use crate::constant::{ SEED_MINTER, SEED_RELAYER_CONFIG, SEED_TOKEN_CONFIG };
use crate::error::ProtocolProgramError;
use crate::state::relayer_config::*;
use crate::state::token_config::TokenConfig;
use anchor_lang::prelude::*;
use anchor_spl::token_2022::{ mint_to, MintTo, Token2022 };
use anchor_spl::token_interface::{ Mint, TokenAccount };

/// Mint token to user ata
#[derive(Accounts)]
pub struct MintToUserAta<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(seeds = [SEED_RELAYER_CONFIG.as_ref()], bump)]
    pub relayer_config: Box<Account<'info, RelayerConfig>>,

    #[account(seeds = [SEED_TOKEN_CONFIG.as_ref()], bump)]
    pub token_config: Box<Account<'info, TokenConfig>>,

    #[account(seeds = [SEED_MINTER.as_ref()], bump)]
    pub minter: AccountInfo<'info>,

    #[account(mut, address = token_config.mint)]
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    /// CHECK:
    #[account()]
    pub user: AccountInfo<'info>,

    #[account(mut, 
        associated_token::mint = mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Program<'info, Token2022>,
    // pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn mint_to_user_ata(ctx: Context<MintToUserAta>, amount: u64) -> Result<()> {
    require!(
        &ctx.accounts.relayer_config.is_relayer(&ctx.accounts.payer.key()),
        ProtocolProgramError::RequireRelayer
    );

    let bump = ctx.bumps.minter;
    let seeds = &[SEED_MINTER.as_ref(), &[bump]];
    let signer: &[&[&[u8]]] = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.minter.to_account_info(),
        },
        signer
    );
    mint_to(cpi_ctx, amount).unwrap();

    msg!(
        "Mint {} token to user {} ata {}",
        amount,
        ctx.accounts.user.key(),
        ctx.accounts.user_token_account.key()
    );

    Ok(())
}

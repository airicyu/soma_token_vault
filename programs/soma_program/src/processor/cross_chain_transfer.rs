use crate::constant::{ SEED_RELAYER_CONFIG, SEED_TOKEN_CONFIG };
use crate::error::ProtocolProgramError;
use crate::state::relayer_config::*;
use crate::state::token_config::TokenConfig;
use crate::events::CrossChainTransferEvent;
use anchor_lang::prelude::*;
use anchor_spl::token_2022::{ burn, Burn, Token2022 };
use anchor_spl::token_interface::{ Mint, TokenAccount };

/// Cross Chain Transfer
#[derive(Accounts)]
#[instruction(data: Box<CrossChainTransferInstructionData>)]
pub struct CrossChainTransferContext<'info> {
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

    #[account(mut, address = token_config.mint)]
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    /// CHECK:
    #[account(mut, address = data.user)]
    pub user: Signer<'info>,

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

#[account]
pub struct CrossChainTransferInstructionData {
    pub user: Pubkey,
    pub from_chain_id: u32,
    pub to_chain_id: u32,
    pub amount: u64,
}

pub fn cross_chain_transfer(
    ctx: Context<CrossChainTransferContext>,
    data: CrossChainTransferInstructionData
) -> Result<()> {
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), Burn {
        mint: ctx.accounts.mint.to_account_info(),
        from: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    });
    burn(cpi_ctx, data.amount).unwrap();

    msg!(
        "Cross chain transfer burn {} token from user {} ata {}",
        data.amount,
        data.user,
        ctx.accounts.user_token_account.key()
    );

    emit!(CrossChainTransferEvent {
        user: data.user.key(),
        from_chain_id: data.from_chain_id,
        to_chain_id: data.to_chain_id,
        amount: data.amount,
    });

    Ok(())
}

use crate::constant::{ SEED_MINTER, SEED_RELAYER_CONFIG, SEED_TOKEN_CONFIG };
use crate::error::ProtocolProgramError;
use crate::state::relayer_config::*;
use crate::state::token_config::TokenConfig;
use crate::events::CrossChainReceivedEvent;
use anchor_lang::prelude::*;
use anchor_spl::token_2022::{ mint_to, MintTo, Token2022 };
use anchor_spl::token_interface::{ Mint, TokenAccount };

/// Cross Chain Receive
#[derive(Accounts)]
#[instruction(data: Box<CrossChainReceiveInstructionData>)]
pub struct CrossChainReceiveContext<'info> {
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

    /// CHECK:
    #[account(address = data.user)]
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

#[account]
pub struct CrossChainReceiveInstructionData {
    pub user: Pubkey,
    pub from_chain_id: u32,
    pub to_chain_id: u32,
    pub amount: u64,
    pub ref_id: String,
}

pub fn cross_chain_receive(
    ctx: Context<CrossChainReceiveContext>,
    data: CrossChainReceiveInstructionData
) -> Result<()> {
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
    mint_to(cpi_ctx, data.amount).unwrap();

    msg!(
        "Cross chain received Mint {} token to user {} ata {}",
        data.amount,
        data.user,
        ctx.accounts.user_token_account.key()
    );

    emit!(CrossChainReceivedEvent {
        user: data.user.key(),
        from_chain_id: data.from_chain_id,
        to_chain_id: data.to_chain_id,
        amount: data.amount,
        ref_id: data.ref_id,
    });

    Ok(())
}

use anchor_lang::prelude::*;

mod constant;
mod error;
mod state;
mod processor;
use processor::*;
mod utils;

declare_id!("5KdZXi6qytcuuGHuDwsUbznjm5eArggdDiyhE1k1CNp1");

#[program]
pub mod soma_program {
    use super::*;

    pub fn initialize_program(
        ctx: Context<InitializeProgram>,
        data: Box<InitializeProgramInstructionData>
    ) -> Result<()> {
        processor::initialize_program(ctx, data)
    }

    pub fn initialize_token(
        ctx: Context<InitializeToken>,
        data: Box<InitializeTokenInstructionData>
    ) -> Result<()> {
        processor::initialize_token(ctx, data)
    }

    pub fn change_owner(ctx: Context<ChangeOwner>, owner: Pubkey) -> Result<()> {
        processor::change_owner(ctx, owner)
    }

    pub fn change_relayer(ctx: Context<ChangeRelayer>, relayer: Pubkey) -> Result<()> {
        processor::change_relayer(ctx, relayer)
    }

    pub fn change_stake_rate(ctx: Context<ChangeStakeRate>, stake_rate: u32) -> Result<()> {
        processor::change_stake_rate(ctx, stake_rate)
    }

    pub fn mint_to_vault(ctx: Context<MintToVault>, amount: u64) -> Result<()> {
        processor::mint_to_vault(ctx, amount)
    }

    pub fn burn_from_vault(ctx: Context<BurnFromVault>, amount: u64) -> Result<()> {
        processor::burn_from_vault(ctx, amount)
    }

    pub fn mint_to_user_ata(ctx: Context<MintToUserAta>, amount: u64) -> Result<()> {
        processor::mint_to_user_ata(ctx, amount)
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        processor::stake(ctx, amount)
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        processor::unstake(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        processor::withdraw(ctx)
    }
}

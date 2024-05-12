use anchor_lang::prelude::*;

mod constant;
mod error;
mod state;
mod events;
mod processor;
use processor::*;
mod utils;

declare_id!("5KdZXi6qytcuuGHuDwsUbznjm5eArggdDiyhE1k1CNp1");

#[program]
pub mod soma_program {
    use super::*;

    pub fn initialize_program(
        ctx: Context<InitializeProgramContext>,
        data: Box<InitializeProgramInstructionData>
    ) -> Result<()> {
        processor::initialize_program(ctx, data)
    }

    pub fn initialize_token(
        ctx: Context<InitializeTokenContext>,
        data: Box<InitializeTokenInstructionData>
    ) -> Result<()> {
        processor::initialize_token(ctx, data)
    }

    pub fn change_owner(ctx: Context<ChangeOwnerContext>, owner: Pubkey) -> Result<()> {
        processor::change_owner(ctx, owner)
    }

    pub fn change_relayer(ctx: Context<ChangeRelayerContext>, relayer: Pubkey) -> Result<()> {
        processor::change_relayer(ctx, relayer)
    }

    pub fn change_stake_rate(ctx: Context<ChangeStakeRateContext>, stake_rate: u32) -> Result<()> {
        processor::change_stake_rate(ctx, stake_rate)
    }

    pub fn mint_to_vault(ctx: Context<MintToVaultContext>, amount: u64) -> Result<()> {
        processor::mint_to_vault(ctx, amount)
    }

    pub fn burn_from_vault(ctx: Context<BurnFromVaultContext>, amount: u64) -> Result<()> {
        processor::burn_from_vault(ctx, amount)
    }

    pub fn mint_to_user_ata(ctx: Context<MintToUserAtaContext>, amount: u64) -> Result<()> {
        processor::mint_to_user_ata(ctx, amount)
    }

    pub fn stake(ctx: Context<StakeContext>, amount: u64) -> Result<()> {
        processor::stake(ctx, amount)
    }

    pub fn unstake(ctx: Context<UnstakeContext>, amount: u64) -> Result<()> {
        processor::unstake(ctx, amount)
    }

    pub fn withdraw(ctx: Context<WithdrawContext>) -> Result<()> {
        processor::withdraw(ctx)
    }

    pub fn cross_chain_receive(
        ctx: Context<CrossChainReceiveContext>,
        data: CrossChainReceiveInstructionData
    ) -> Result<()> {
        processor::cross_chain_receive(ctx, data)
    }

    pub fn cross_chain_transfer(
        ctx: Context<CrossChainTransferContext>,
        data: CrossChainTransferInstructionData
    ) -> Result<()> {
        processor::cross_chain_transfer(ctx, data)
    }
}

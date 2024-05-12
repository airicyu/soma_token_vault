use anchor_lang::prelude::*;

#[event]
pub struct CrossChainReceivedEvent {
    #[index]
    pub user: Pubkey,
    pub from_chain_id: u32,
    pub to_chain_id: u32,
    pub amount: u64,
    pub ref_id: String,
}

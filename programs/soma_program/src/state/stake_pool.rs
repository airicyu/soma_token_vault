use anchor_lang::prelude::*;

/// Stake pool account
///
/// The stake pool account is used to store the total balance of the pool and the base rate of the pool.
#[account]
pub struct StakePool {
    pub balance: u64,
    pub stake_base_rate: u32,
}

impl StakePool {
    pub const INIT_SPACE: usize = 8 + 4;
}

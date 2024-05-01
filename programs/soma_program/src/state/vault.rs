use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub balance: u64,
}

impl Vault {
    pub const INIT_SPACE: usize = 8;
}

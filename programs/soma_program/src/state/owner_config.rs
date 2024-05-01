use anchor_lang::prelude::*;

/// The own of the program
#[account]
pub struct OwnerConfig {
    pub owner: Pubkey,
}

impl OwnerConfig {
    pub fn is_owner(&self, user: &Pubkey) -> bool {
        self.owner == *user
    }

    pub const INIT_SPACE: usize = 32;
}

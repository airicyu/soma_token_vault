use anchor_lang::prelude::*;

/// The relayer of the program
#[account]
pub struct RelayerConfig {
    pub relayer: Pubkey,
}

impl RelayerConfig {
    pub fn is_relayer(&self, user: &Pubkey) -> bool {
        self.relayer == *user
    }

    pub const INIT_SPACE: usize = 32;
}

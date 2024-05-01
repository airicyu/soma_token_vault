use anchor_lang::prelude::*;

/// Token config account
///
/// The token config account is used to store the mint of the token.
#[account]
pub struct TokenConfig {
    pub mint: Pubkey,
}

impl TokenConfig {
    pub const INIT_SPACE: usize = 32;
}

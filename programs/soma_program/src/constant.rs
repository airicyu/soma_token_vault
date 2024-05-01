use anchor_lang::constant;

#[constant]
pub const SEED_OWNER_CONFIG: &'static [u8] = b"owner_config";

#[constant]
pub const SEED_RELAYER_CONFIG: &'static [u8] = b"relayer_config";

#[constant]
pub const SEED_TOKEN_CONFIG: &'static [u8] = b"token_config";

#[constant]
pub const SEED_MINT: &'static [u8] = b"mint_token:soma";

#[constant]
pub const SEED_MINTER: &'static [u8] = b"minter";

#[constant]
pub const SEED_VAULT: &'static [u8] = b"vault";

#[constant]
pub const SEED_VAULT_TOKEN_ACCOUNT: &'static [u8] = b"vault_token_account";

#[constant]
pub const SEED_STAKE_POOL: &'static [u8] = b"stake_pool";

#[constant]
pub const SEED_STAKE_POOL_TOKEN_ACCOUNT: &'static [u8] = b"stake_pool_token_account";

#[constant]
pub const SEED_PREFIX_USER_STAKE_INFO: &'static [u8] = b"user_stake_info:"; // stake_info:mint:user

pub static SLOT_PER_YEAR: u64 = (365u64 * 86400u64 * 1000u64) / 400u64;

pub static LOCK_PERIOD_SLOTS: u64 = 0; //(30u64 * 86400u64 * 1000u64) / 400u64;

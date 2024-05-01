use anchor_lang::prelude::error_code;

#[error_code]
pub enum ProtocolProgramError {
    #[msg("RequireOwner")]
    RequireOwner,

    #[msg("RequireRelayer")]
    RequireRelayer,

    #[msg("RequireDataInitialized")]
    RequireDataInitialized,

    #[msg("UserStakeInfoNotInialized")]
    UserStakeInfoNotInialized,

    #[msg("NoBalanceToUnstake")]
    NoBalanceToUnstake,

    #[msg("NotEnoughBalanceToUnstake")]
    NotEnoughBalanceToUnstake,

    #[msg("CannotWithdrawLockingAsset")]
    CannotWithdrawLockingAsset,

    #[msg("NoBalanceToWithdraw")]
    NoBalanceToWithdraw,
}

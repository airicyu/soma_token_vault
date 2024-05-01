use anchor_lang::prelude::*;
use crate::utils::apy_calculate::calculate_stake_balance;
use crate::error::ProtocolProgramError;
use std::result::Result;

/// User stake info account
///
/// The user stake info account is used to store the user's staked balance and reward balance.
#[account]
pub struct UserStakeInfo {
    pub is_initialized: bool,
    pub token_balance: u64, // staked tokens
    pub reward_balance: u64, // pending mint rewards

    pub stake_start_slot: u64, // the beginning slot, just for reference to view only
    pub last_calibrate_slot: u64, // the last slot that calibrated balance rewards

    pub lock_balance: u64, // pending withdraw tokens
    pub lock_reward_balance: u64, // pending mint rewards
    pub lock_until_slot: u64,
}

impl UserStakeInfo {
    pub const INIT_SPACE: usize = 1 + 8 + 8 + 8 + 8 + 8 + 8 + 8;

    // Reset stake as unintialized
    pub fn reset(&mut self) {
        self.is_initialized = false;
        self.token_balance = 0;
        self.reward_balance = 0;
        self.stake_start_slot = 0;
        self.last_calibrate_slot = 0;
        self.lock_balance = 0;
        self.lock_reward_balance = 0;
        self.lock_until_slot = 0;
    }

    // Initial stake as initialized
    pub fn initialize(&mut self) {
        self.reset();
        self.is_initialized = true;
    }

    /// check is locking
    pub fn is_locking(&self, current_slot: u64) -> bool {
        self.lock_until_slot > current_slot
    }

    /// Recalibrate the reward balance
    pub fn calibrate_balance(&mut self, stake_rate: u32, current_slot: u64) {
        if self.is_initialized == false {
            panic!("UserStakeInfo not initialized");
        }

        let reward = calculate_stake_balance(
            self.token_balance.checked_add(self.reward_balance).unwrap(),
            stake_rate,
            current_slot.checked_sub(self.last_calibrate_slot).unwrap()
        ).unwrap();

        msg!("UserStakeInfo calibrate_balance rate: {}, reward: {}", stake_rate, reward);

        self.last_calibrate_slot = current_slot;

        if reward <= 0 {
            return;
        }
        self.reward_balance = self.reward_balance.checked_add(reward).unwrap();
    }

    /// Initial Stake tokens
    pub fn initial_stake(
        &mut self,
        amount: u64,
        current_slot: u64
    ) -> Result<(), ProtocolProgramError> {
        if self.is_initialized == false {
            return Err(ProtocolProgramError::UserStakeInfoNotInialized);
        }
        self.token_balance = self.token_balance.checked_add(amount).unwrap();
        self.stake_start_slot = current_slot;
        self.last_calibrate_slot = current_slot;

        return Ok(());
    }

    /// Stake tokens
    pub fn stake(
        &mut self,
        amount: u64,
        stake_rate: u32,
        current_slot: u64
    ) -> Result<(), ProtocolProgramError> {
        if self.is_initialized == false {
            return Err(ProtocolProgramError::UserStakeInfoNotInialized);
        }

        self.calibrate_balance(stake_rate, current_slot);
        self.token_balance = self.token_balance.checked_add(amount).unwrap();

        return Ok(());
    }

    /// Unstake tokens, need wait a certain locking period to withdraw
    pub fn unstake(
        &mut self,
        unstake_amount: u64,
        stake_rate: u32,
        current_slot: u64,
        lock_until_slot: u64
    ) -> Result<(), ProtocolProgramError> {
        if self.is_initialized == false {
            return Err(ProtocolProgramError::UserStakeInfoNotInialized);
        }

        // do calibrate to count rewards
        self.calibrate_balance(stake_rate, current_slot);

        if self.token_balance.checked_add(self.reward_balance).unwrap() < unstake_amount {
            return Err(ProtocolProgramError::NoBalanceToUnstake);
        }

        // then mark locked balance & rewards & lock until slot
        let deduct_reward;
        let deduct_balance;
        if unstake_amount <= self.reward_balance {
            deduct_reward = unstake_amount;
            deduct_balance = 0u64;
        } else {
            deduct_reward = self.reward_balance;
            deduct_balance = unstake_amount.checked_sub(self.reward_balance).unwrap();
        }

        self.lock_balance = self.lock_balance.checked_add(deduct_balance).unwrap();
        self.token_balance = self.token_balance.checked_sub(deduct_balance).unwrap();

        self.lock_reward_balance = self.lock_reward_balance.checked_add(deduct_reward).unwrap();
        self.reward_balance = self.reward_balance.checked_sub(deduct_reward).unwrap();

        self.lock_until_slot = lock_until_slot;

        return Ok(());
    }

    /// Withdraw from unstaked tokens
    pub fn withdraw_from_unstaked(
        &mut self,
        current_slot: u64
    ) -> Result<(), ProtocolProgramError> {
        if self.is_initialized == false {
            return Err(ProtocolProgramError::UserStakeInfoNotInialized);
        }

        // check is locking
        if current_slot < self.lock_until_slot {
            return Err(ProtocolProgramError::CannotWithdrawLockingAsset);
        }

        // clear marked withdraw info
        self.lock_balance = 0;
        self.lock_reward_balance = 0;
        self.lock_until_slot = 0;

        // if cleared all, reset account
        if self.token_balance == 0 && self.reward_balance == 0 {
            self.reset();
        }
        return Ok(());
    }

    pub fn println(&self) {
        println!(
            "UserStakeInfo( is_initialized: {}, token_balance: {}, reward_balance: {}, stake_start_slot: {}, last_calibrate_slot: {}\
                , lock_balance: {}, lock_start_slot: {}, lock_until_slot: {} )",
            self.is_initialized,
            self.token_balance,
            self.reward_balance,

            self.stake_start_slot,
            self.last_calibrate_slot,

            self.lock_balance,
            self.lock_reward_balance,
            self.lock_until_slot
        );
    }
}

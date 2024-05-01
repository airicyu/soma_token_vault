use crate::constant::SLOT_PER_YEAR;

/// This function calculates the stake balance.
///
/// # Arguments
///
/// * `stake_amount` - The amount of stake.
/// * `stake_rate` - The rate of stake.
/// * `stake_slot_diff` - The difference in stake slots.
///
/// # Returns
///
/// This function returns an `Option<u64>` representing the stake interest.
pub fn calculate_stake_balance(
    stake_amount: u64,
    stake_rate: u32,
    stake_slot_diff: u64
) -> Option<u64> {
    if stake_amount == 0 || stake_slot_diff <= 0 {
        return Some(0u64);
    }

    // use u128 calculate to prevent intermediate value overflow
    let stake_interest = (stake_amount as u128)
        .checked_mul(stake_rate as u128)
        .and_then(|res| res.checked_mul(stake_slot_diff as u128))
        .and_then(|res| res.checked_div(10000u128))
        .and_then(|res| res.checked_div(SLOT_PER_YEAR as u128));
    stake_interest.map(|res| res as u64)
}

use crate::{errors::MadscapeError, state::Match};
use anchor_lang::prelude::*;

/// Calculates the fee for a given amount by basis points.
///
/// # Arguments
///
/// * `amount` - The amount to calculate the fee for.
/// * `fee_lamports_basis_points` - The fee in basis points.
///
/// # Returns
///
/// * `Result<u64>` - The fee in lamports.
///
/// # Errors
///
/// * If the calculation fails.
///
/// # Example
///
/// ```
/// use madscape::tools::maths::calc_fee_basis_points;
/// // This would be 5.3 SOL.
/// let fee = calc_fee_basis_points(5_300_000_000, 1000);
///  // This would be 0.53 SOL.
/// assert_eq!(fee, Ok(530_000_000));
/// ```
pub fn calc_fee_basis_points(amount: u64, fee_lamports_basis_points: u16) -> Result<u64> {
    let result = amount
        .checked_mul(fee_lamports_basis_points.into())
        .and_then(|product| product.checked_div(10000))
        .ok_or(MadscapeError::FeeCalculationFailure)?;
    Ok(result)
}

/// Calculates the total amount to deduct from the escrow account.
///
/// Math notation:
///
/// ```text
/// total = user_a_target_amount + user_b_target_amount
/// ```
///
/// # Arguments
///
/// * `escrow` - The escrow account.
///
/// # Returns
///
/// * `Result<u64>` - The total amount to deduct from the escrow account.
///
/// # Errors
///
/// * If the calculation fails.
pub fn get_match_total_lamports_checked(game: &mut Account<'_, Match>) -> Result<u64> {
    let total_to_deduct_from_match = game
        .target_amount.checked_mul(2)
        .ok_or(MadscapeError::NumericOverflow)?;
    Ok(total_to_deduct_from_match)
}

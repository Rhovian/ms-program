use anchor_lang::prelude::*;
use crate::{
    constants::{MATCH},
    state::Match,
    errors::MadscapeError,
};

#[derive(Accounts)]
pub struct CancelOpenMatch<'info> {
    #[account(
        mut,
        seeds = [MATCH.as_ref(), user_a.key().as_ref(), mint.key().as_ref()],
        bump = game.bump,
    )]
    pub game: Account<'info, Match>,
    #[account(
        address = game.user_a,
    )]
    pub user_a: Signer<'info>,
    #[account(
        address = game.mint
    )]
    /// CHECK: No check needed here.
    pub mint: AccountInfo<'info>,
}

pub fn cancel_open_match(ctx: Context<CancelOpenMatch>) -> Result<()> {
    let game = &mut ctx.accounts.game;

    if game.user_b != Pubkey::default() {
        return Err(MadscapeError::EscrowNotActivated.into());
    }

    let user_a = &mut ctx.accounts.user_a;

    let total_to_deduct_from_match = game.target_amount;
    let match_deduction_result = game
        .to_account_info()
        .lamports()
        .checked_sub(total_to_deduct_from_match)
        .ok_or(MadscapeError::NumericOverflow)?;
    **game.to_account_info().try_borrow_mut_lamports()? = match_deduction_result;

    let user_a_addition_result = user_a
        .lamports()
        .checked_add(total_to_deduct_from_match)
        .ok_or(MadscapeError::NumericOverflow)?;
    **user_a.try_borrow_mut_lamports()? = user_a_addition_result;

    game.completed = true;

    Ok(())
}

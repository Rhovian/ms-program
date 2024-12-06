use anchor_lang::prelude::*;
use crate::{
    constants::{MATCH, RELEASE},
    state::{Match, ReleaseAuthority},
    errors::MadscapeError,
};

#[derive(Accounts)]
pub struct CancelPrivateMatch<'info> {
    #[account(
        mut,
        seeds = [MATCH.as_ref(), user_a.key().as_ref(), mint.key().as_ref()],
        bump = game.bump,
    )]
    pub game: Account<'info, Match>,
    #[account(
        seeds = [RELEASE.as_ref(), release_authority.authority.key().as_ref()],
        bump = release_authority.bump,
    )]
    pub release_authority: Account<'info, ReleaseAuthority>,
    /// CHECK: no need to check this here.
    #[account(
        mut,
        address = game.user_a,
    )]
    pub user_a: AccountInfo<'info>,
    #[account(
        address = release_authority.authority,
    )]
    pub signer: Signer<'info>,
    #[account(
        address = game.mint
    )]
    /// CHECK: No check needed here.
    pub mint: AccountInfo<'info,>,
}

pub fn cancel_private_match(ctx: Context<CancelPrivateMatch>) -> Result<()> {
    // TODO: check constraint - user b must be set
    // TODO: burn match mint from user using the match pda
    // TODO: return funds to user_a
    // TODO: close match pda account
    let game = &mut ctx.accounts.game;
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

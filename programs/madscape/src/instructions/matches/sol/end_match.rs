use anchor_lang::prelude::*;
use crate::{
    constants::{MATCH, RELEASE},
    errors::MadscapeError,
    state::{Match, ReleaseAuthority},
    maths::get_match_total_lamports_checked
};

#[derive(Accounts)]
pub struct EndMatch<'info> {
    #[account(
        mut,
        seeds = [
            MATCH.as_ref(),
            user_a.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump = game.bump,
        constraint = game.active @ MadscapeError::EscrowNotActive,
        constraint = game.is_native_sol() @ MadscapeError::EscrowNotNativeSol
    )]
    pub game: Account<'info, Match>,
    #[account(
        seeds = [RELEASE.as_ref(), release_authority.authority.key().as_ref()],
        bump = release_authority.bump,
    )]
    pub release_authority: Account<'info, ReleaseAuthority>,

    /// CHECK: No additional checks needed here.
    #[account(
        mut,
        address = game.user_a
    )]
    pub user_a: AccountInfo<'info>,

    /// CHECK: No additional checks needed here.
    #[account(
        mut,
        address = game.user_b,
    )]
    pub user_b: AccountInfo<'info>,

    #[account(address = release_authority.authority)]
    pub signer: Signer<'info>,
    /// CHECK: No check
    #[account(
        mut,
        address = release_authority.treasury,
    )]
    pub treasury: AccountInfo<'info>,
    #[account(
        address = game.mint
    )]
    /// CHECK: No check needed here.
    pub mint: AccountInfo<'info>,
}

pub fn end_match(ctx: Context<EndMatch>, winner: Pubkey) -> Result<()> {
    let game: &mut Account<'_, Match> = &mut ctx.accounts.game;
    let user_a = &ctx.accounts.user_a;
    let treasury = &mut ctx.accounts.treasury;
    let user_b = &ctx.accounts.user_b;

    let (winner, winner_target_amount) = match winner {
        winner if winner == user_a.key() => (
            user_a,
            game.target_amount,
        ),
        winner if winner == user_b.key() => (
            user_b,
            game.target_amount,
        ),
        _ => return Err(MadscapeError::InvalidWinner.into()),
    };


    let total_deduct_from_game = get_match_total_lamports_checked(game)?;
    let fee_amount = total_deduct_from_game
        .checked_mul(5)
        .ok_or(MadscapeError::NumericOverflow)?
        .checked_div(100)
        .ok_or(MadscapeError::NumericOverflow)?;

    let game_deduction_result = game
        .to_account_info()
        .lamports()
        .checked_sub(total_deduct_from_game)
        .ok_or(MadscapeError::NumericOverflow)?;
    **game.to_account_info().try_borrow_mut_lamports()? = game_deduction_result;

    let winner_amount = total_deduct_from_game
        .checked_sub(fee_amount)
        .ok_or(MadscapeError::NumericOverflow)?;

    let winner_addition_result = winner
        .to_account_info()
        .lamports()
        .checked_add(winner_amount)
        .ok_or(MadscapeError::NumericOverflow)?;
    **winner.to_account_info().try_borrow_mut_lamports()? = winner_addition_result;

    msg!("Winner: {:?}", winner.key());
    msg!("Winner Amount: {:?}", winner_amount);
    msg!("Fee Amount: {:?}", fee_amount);

    let treasury_amount = treasury
        .to_account_info()
        .lamports()
        .checked_add(fee_amount)
        .ok_or(MadscapeError::NumericOverflow)?;
    **treasury.to_account_info().try_borrow_mut_lamports()? = treasury_amount;

    // TODO: close escrow account and add it to fee pool
    // TODO: burn user nft

    game.completed = true;

    Ok(())
}

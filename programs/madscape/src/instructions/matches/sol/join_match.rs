use anchor_lang::{prelude::*, solana_program::program::invoke};
use solana_program::system_instruction;
use crate::{
    constants::{MATCH, RELEASE},
    errors::MadscapeError,
    state::{Match, ReleaseAuthority},
};

#[derive(Accounts)]
pub struct JoinMatch<'info> {
    #[account(
        mut,
        seeds = [
            MATCH.as_ref(),
            user_a.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump = game.bump,
        constraint = !game.active @ MadscapeError::EscrowIsActive,
        constraint = game.is_initialized() @ MadscapeError::EscrowNotInitialized,
        constraint = game.is_native_sol() @ MadscapeError::EscrowNotNativeSol
    )]
    pub game: Account<'info, Match>,
    #[account(
        seeds = [RELEASE.as_ref(), release_authority.authority.key().as_ref()],
        bump = release_authority.bump,
    )]
    pub release_authority: Account<'info, ReleaseAuthority>,
    /// CHECK: No additional checks needed here.
    #[account(address = game.user_a)]
    pub user_a: AccountInfo<'info>,
    pub user_b: Signer<'info>,
    #[account(
        address = game.mint,
    )]
    /// CHECK: No additional checks needed here.
    pub mint: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn join_match(ctx: Context<JoinMatch>) -> Result<()> {
    let game = &mut ctx.accounts.game;
    let fee_lamports = game.fee_amount;
    let user_b = &ctx.accounts.user_b;

    if game.user_b != Pubkey::default() {
        require!(
            game.user_b == ctx.accounts.user_b.key(),
            MadscapeError::InvalidUserB
        );
    } else {
        game.user_b = ctx.accounts.user_b.key();
    }

    // Transfer fee_lamports from user_b to game
    invoke(
        &system_instruction::transfer(&ctx.accounts.user_b.key(), &game.key(), fee_lamports),
        &[
            ctx.accounts.user_b.to_account_info(),
            game.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    // Transfer funds from user_a to game
    let user_amount = game
        .target_amount
        .checked_sub(fee_lamports)
        .ok_or(MadscapeError::NumericOverflow)?;
    invoke(
        &system_instruction::transfer(
            &ctx.accounts.user_b.key(),
            &game.key(),
            user_amount,
        ),
        &[
            ctx.accounts.user_b.to_account_info(),
            game.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;

    if game.user_b == Pubkey::default() {
        game.activate_public(user_b.key(), current_timestamp);
    } else {
        game.activate(current_timestamp);
    }

    Ok(())
}

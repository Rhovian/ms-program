use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use crate::{
    constants::{MATCH},
    errors::MadscapeError,
    state::Match,
};

#[derive(Accounts)]
pub struct CancelOpenMatchMint<'info> {
    #[account(
        mut,
        seeds = [
            MATCH.as_ref(),
            user_a.key().as_ref(),
            mint.key().as_ref()
        ],
        bump = game.bump,
        constraint = !game.is_native_sol() @ MadscapeError::EscrowIsNativeSol,
        constraint = game.is_initialized() @ MadscapeError::EscrowNotInitialized,
    )]
    pub game: Account<'info, Match>,
    #[account(
        address = game.mint,
    )]
    /// CHECK: No check needed here.
    pub mint: AccountInfo<'info>,
    // User A Staked Item.
    #[account(
        constraint = match_mint.key() == game.target_mint,
        constraint = match_mint.key() == game.target_mint,
    )]
    pub match_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        constraint = user_a_token_account.mint == match_mint.key(),
        constraint = user_a_token_account.owner == user_a.key(),
    )]
    pub user_a_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        token::mint = match_mint,
        token::authority = game,
    )]
    pub match_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        address = game.user_a,
    )]
    pub user_a: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn cancel_open_match_mint(ctx: Context<CancelOpenMatchMint>) -> Result<()> {
    let game = &mut ctx.accounts.game;
    let user_a_token_account = &mut ctx.accounts.user_a_token_account;
    let match_token_account = &mut ctx.accounts.match_token_account;

    let user_a_key = ctx.accounts.user_a.key();
    let mint = ctx.accounts.mint.key();

    let match_seeds = &[
        MATCH.as_ref(),
        user_a_key.as_ref(),
        mint.as_ref(),
        &[game.bump],
    ];
    let match_signer = &[&match_seeds[..]];

    // Unlock escrowed item.
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                authority: game.to_account_info(),
                from: match_token_account.to_account_info(),
                to: user_a_token_account.to_account_info(),
            },
        )
        .with_signer(match_signer),
        game.target_amount,
    )?;

    game.completed = true;

    // TODO: close out account
    Ok(())
}
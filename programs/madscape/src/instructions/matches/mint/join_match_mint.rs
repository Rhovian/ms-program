use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use crate::{
    constants::{MATCH, RELEASE},
    errors::MadscapeError,
    state::{Match, ReleaseAuthority},
};

#[derive(Accounts)]
pub struct JoinMatchMint<'info> {
    #[account(
        mut,
        seeds = [
            MATCH.as_ref(),
            user_a.key().as_ref(),
            mint.key().as_ref()
        ],
        bump = game.bump,
        constraint = !game.active @ MadscapeError::EscrowIsActive,
        constraint = game.is_initialized() @ MadscapeError::EscrowNotInitialized,
    )]
    pub game: Account<'info, Match>,
    #[account(
        seeds = [RELEASE.as_ref(), release_authority.authority.key().as_ref()],
        bump = release_authority.bump,
    )]
    pub release_authority: Account<'info, ReleaseAuthority>,
    /// CHECK: No use checking this.
    #[account(address = game.user_a)]
    pub user_a: AccountInfo<'info>,
    #[account(
        constraint = match_mint.key() == game.target_mint,
        constraint = match_mint.key() == game.target_mint,
    )]
    pub match_mint: Box<Account<'info, Mint>>,
    // NOTE: This account must be pre-existing, and is not the responsibility of the program
    // Creating it, only it's own recipient for this escrow instance.
    #[account(
        mut,
        constraint = user_b_token_account.mint == match_mint.key(),
        constraint = user_b_token_account.owner == user_b.key(),
    )]
    pub user_b_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        token::mint = match_mint,
        token::authority = game,
    )]
    pub match_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: No use checking this.
    #[account(mut)]
    pub user_b: Signer<'info>,
    #[account(
        address = game.mint,
    )]
    /// CHECK: No check needed here.
    pub mint: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn join_match_mint(
    ctx: Context<JoinMatchMint>,
) -> Result<()> {
    let game = &mut ctx.accounts.game;
    let release_authority = &ctx.accounts.release_authority;
    let user_b = &ctx.accounts.user_b;
    let user_b_token_account = &mut ctx.accounts.user_b_token_account;
    let match_token_account = &mut ctx.accounts.match_token_account;
    let match_mint = &ctx.accounts.match_mint;

    let matching_fee_mint = release_authority
        .approved_mints
        .iter()
        .find(|m| m.mint == match_mint.key());
    require!(matching_fee_mint.is_some(), MadscapeError::InvalidFeeMint);

    let amount = game.target_amount;
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: user_b_token_account.to_account_info(),
                to: match_token_account.to_account_info(),
                authority: user_b.to_account_info(),
            },
        ),
        amount,
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

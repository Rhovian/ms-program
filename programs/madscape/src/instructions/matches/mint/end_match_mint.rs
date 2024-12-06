use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use crate::{
    constants::{
        MATCH, RELEASE
    },
    errors::MadscapeError,
    state::{Match, ReleaseAuthority},
};

#[derive(Accounts)]
pub struct EndMatchMint<'info> {
    #[account(
        mut,
        seeds = [
            MATCH.as_ref(),
            user_a.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump = game.bump,
        constraint = game.active @ MadscapeError::EscrowNotActive,
        constraint = !game.is_native_sol() @ MadscapeError::EscrowIsNativeSol
    )]
    pub game: Account<'info, Match>,
    #[account(
        seeds = [RELEASE.as_ref(), release_authority.authority.key().as_ref()],
        bump = release_authority.bump,
    )]
    pub release_authority: Account<'info, ReleaseAuthority>,
    /// CHECK: No check needed
    #[account(
        address = game.user_a
    )]
    pub user_a: AccountInfo<'info>,
    #[account(
        constraint = match_mint.key() == game.target_mint,
    )]
    pub match_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        token::mint = match_mint,
        token::authority = game,
    )]
    pub match_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: No check needed
    #[account(
        address = game.user_b
    )]
    pub user_b: AccountInfo<'info>,
    /// CHECK: No check needed
    #[account(
        address = game.mint,
    )]
    pub mint: AccountInfo<'info>,
    /// CHECK: No check needed
    #[account(
        address = release_authority.treasury,
    )]
    pub treasury: AccountInfo<'info>,
    #[account(
        mut,
        token::mint = match_mint,
        token::authority = treasury,
    )]
    pub treasury_fee_recipient_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        token::mint = match_mint,
        token::authority = winner,
    )]
    pub winner_recipient_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: checked in program
    pub winner: AccountInfo<'info>,
    #[account(
        mut, // Concern: Signer now needs to pay sol for the account initialization.
        address = release_authority.authority,
    )]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn end_match_mint(ctx: Context<EndMatchMint>) -> Result<()> {
    let game = &mut ctx.accounts.game;
    let treasury_fee_recipient_token_account =
        &mut ctx.accounts.treasury_fee_recipient_token_account;

    let match_token_account = &mut ctx.accounts.match_token_account;
    let user_a_key = ctx.accounts.user_a.key();

    if game.user_a == Pubkey::default() || game.user_b == Pubkey::default() {
        return Err(MadscapeError::EscrowNotActive.into());
    }

    let winner_recipient_token_account = &mut ctx.accounts.winner_recipient_token_account;
    let mint = ctx.accounts.mint.key();

    let match_seeds = &[
        MATCH.as_ref(),
        user_a_key.as_ref(),
        mint.as_ref(),
        &[game.bump],
    ];
    let match_signer = &[&match_seeds[..]];

    let fee_amount = (game.target_amount * 2) * 5 / 100;


    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: match_token_account.to_account_info(),
                to: treasury_fee_recipient_token_account.to_account_info(),
                authority: game.to_account_info(),
            },
        )
        .with_signer(match_signer),
        fee_amount,
    )?;


    // Transfer prize from loser to winner.
    // Then transfer winner's stake back to winner.
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: match_token_account.to_account_info(),
                to: winner_recipient_token_account.to_account_info(),
                authority: game.to_account_info(),
            },
        )
        .with_signer(match_signer),
        game.target_amount * 2 -  fee_amount,
    )?;

    game.completed = true;

    Ok(())
}

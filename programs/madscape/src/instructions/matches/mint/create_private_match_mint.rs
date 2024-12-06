use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};
use anchor_spl::associated_token::AssociatedToken;
use solana_program::sysvar::SysvarId;

use crate::{
    constants::{MATCH, RELEASE},
    errors::MadscapeError,
    state::{is_valid_match_type_for_init, Match, ReleaseAuthority},
    strings::generate_escrow_id,
};

#[derive(Accounts)]
pub struct CreatePrivateMatchMint<'info> {
    #[account(
        init,
        payer = user_a,
        space = Match::space(),
        seeds = [
            MATCH.as_ref(),
            user_a.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        constraint = !game.active @ MadscapeError::EscrowIsActive,
        constraint = game.is_not_initialized() @ MadscapeError::EscrowInitialized,
    )]
    pub game: Account<'info, Match>,
    #[account(
        seeds = [RELEASE.as_ref(), release_authority.authority.key().as_ref()],
        bump = release_authority.bump,
    )]
    pub release_authority: Account<'info, ReleaseAuthority>,
    #[account(mut)]
    pub user_a: Signer<'info>,
    #[account(
        mut,
        constraint = user_a_token_account.mint == match_mint.key(),
        constraint = user_a_token_account.owner == user_a.key(),
    )]
    pub user_a_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        init,
        payer = user_a,
        associated_token::mint = match_mint,
        associated_token::authority = game,
    )]
    pub match_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: No check needed here.
    pub mint: AccountInfo<'info>,
    pub match_mint: Box<Account<'info, Mint>>,
    /// CHECK: No use checking this.
    #[account(address = SlotHashes::id())]
    pub recent_slothashes: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>, 
}

pub fn create_private_match_mint(
    ctx: Context<CreatePrivateMatchMint>,
    amount: u64,
    match_type: u8,
    user_b: Pubkey,
) -> Result<()> {
    let game = &mut ctx.accounts.game;
    let release_authority = &ctx.accounts.release_authority;

    let user_a = &ctx.accounts.user_a;
    let user_a_token_account = &mut ctx.accounts.user_a_token_account;
    let match_token_account = &mut ctx.accounts.match_token_account;
    let match_mint = &ctx.accounts.match_mint;

    let match_bump = ctx.bumps.game;
    let mint = &ctx.accounts.mint;


    let recent_slothashes = &ctx.accounts.recent_slothashes;

    require!(
        is_valid_match_type_for_init(match_type),
        MadscapeError::InvalidMatchType
    );

    let matching_fee_mint = release_authority
        .approved_mints
        .iter()
        .find(|m| m.mint == match_mint.key());
    require!(matching_fee_mint.is_some(), MadscapeError::InvalidFeeMint);
    let matching_fee_mint = matching_fee_mint.unwrap();

    // Transfer token to escrow.
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: user_a_token_account.to_account_info(),
                to: match_token_account.to_account_info(),
                authority: user_a.to_account_info(),
            },
        ),
        amount,
    )?;

    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;

    game.set_inner(Match::new_private(
        match_bump,
        release_authority.key(),
        user_a.key(),
        user_b,
        mint.key(),
        current_timestamp,
    ));

    game.init(
        matching_fee_mint.fee,
        match_mint.key(),
        amount,
        match_type,
        generate_escrow_id(recent_slothashes, clock),
        mint.key()
    );
    
    Ok(())
}

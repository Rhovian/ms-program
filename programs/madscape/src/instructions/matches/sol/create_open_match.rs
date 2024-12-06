use anchor_lang::{prelude::*, solana_program::program::invoke};
use crate::{
    constants::{MATCH, RELEASE, SOLANA_PUBKEY},
    state::{Match, ReleaseAuthority, is_valid_match_type_for_init},
    strings::generate_escrow_id,
    errors::MadscapeError,
    maths::calc_fee_basis_points,
};
use solana_program::{slot_hashes::SlotHashes, system_instruction, sysvar::SysvarId};


#[derive(Accounts)]
pub struct CreateOpenMatch<'info> {
    #[account(
        init,
        payer = user_a,
        seeds = [MATCH.as_ref(), user_a.key().as_ref(), mint.key().as_ref()],
        bump,
        space = Match::space(),
    )]
    pub game: Account<'info, Match>,
    #[account(
        seeds = [RELEASE.as_ref(), release_authority.authority.key().as_ref()], 
        bump = release_authority.bump,
    )]
    pub release_authority: Account<'info, ReleaseAuthority>,
    /// CHECK: No use checking this.
    pub mint: AccountInfo<'info>,
    #[account(mut)]
    pub user_a: Signer<'info>,
    /// CHECK: No use checking this.
    #[account(address = SlotHashes::id())]
    pub recent_slothashes: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_open_match(ctx: Context<CreateOpenMatch>, amount: u64, match_type: u8) -> Result<()> {
    let user_a = &ctx.accounts.user_a;
    let release_authority = &ctx.accounts.release_authority;
    let game = &mut ctx.accounts.game;
    let match_bump = ctx.bumps.game;
    let recent_slothashes = &ctx.accounts.recent_slothashes;
    let mint = &ctx.accounts.mint;

    let fee_lamports_basis_points = ctx.accounts.release_authority.fee_lamports_basis_points;

    let fee_lamports = calc_fee_basis_points(amount, fee_lamports_basis_points)?;

    require!(
        is_valid_match_type_for_init(match_type),
        MadscapeError::InvalidMatchType
    );

    // Transfer fee_lamports from user_a to escrow
    invoke(
        &system_instruction::transfer(&ctx.accounts.user_a.key(), &game.key(), fee_lamports),
        &[
            ctx.accounts.user_a.to_account_info(),
            game.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    // Transfer funds from user_a to escrow
    let user_amount = amount
        .checked_sub(fee_lamports)
        .ok_or(MadscapeError::NumericOverflow)?;
    invoke(
        &system_instruction::transfer(&ctx.accounts.user_a.key(), &game.key(), user_amount),
        &[
            ctx.accounts.user_a.to_account_info(),
            game.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;
    
    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;

    game.set_inner(Match::new(
        match_bump,
        release_authority.key(),
        user_a.key(),
        mint.key(),
        current_timestamp
    ));

    game.init(
        fee_lamports,
        SOLANA_PUBKEY,
        amount,
        match_type,
        generate_escrow_id(recent_slothashes, clock),
        mint.key(),
    );

    Ok(())
}

use anchor_lang::prelude::*;
use crate::{
    constants::{MATCH, RELEASE},
    state::{Match, ReleaseAuthority}
};
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct CloseMatch<'info> {
    #[account(
        mut,
        address = release_authority.authority,
    )]
    pub authority: Signer<'info>,
    #[account(
        mut,
        close = destination,
        seeds = [MATCH.as_ref(), user_a.key().as_ref(), mint.key().as_ref()],
        bump = game.bump,
    )]
    pub game: Account<'info, Match>,
    #[account(
        seeds = [RELEASE.as_ref(), release_authority.authority.key().as_ref()], 
        bump = release_authority.bump,
    )]
    pub release_authority: Account<'info, ReleaseAuthority>,
    pub mint: Account<'info, Mint>,
    /// CHECK: No use checking this.
    pub destination: AccountInfo<'info>,
    #[account(
        address = game.user_a,
    )]
    /// CHECK: No use checking this.
    pub user_a: AccountInfo<'info>,
}

pub fn close_match(_ctx: Context<CloseMatch>) -> Result<()> {
    Ok(())
}

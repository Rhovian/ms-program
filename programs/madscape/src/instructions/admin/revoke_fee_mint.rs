use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::{constants::RELEASE, state::ReleaseAuthority, errors::MadscapeError};

#[derive(Accounts)]
pub struct RevokeFeeMint<'info> {
    #[account(
        mut,
        seeds = [RELEASE.as_ref(), authority.key().as_ref()], 
        bump = release_authority.bump,
    )]
    pub release_authority: Account<'info, ReleaseAuthority>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn revoke_fee_mint(ctx: Context<RevokeFeeMint>) -> Result<()> {
    let release_authority = &mut ctx.accounts.release_authority;
    let mint_key = &ctx.accounts.mint.key();
    require!(
        release_authority
            .approved_mints
            .iter()
            .any(|i| &i.mint == mint_key),
        MadscapeError::FeeMintNotApproved
    );
    release_authority.revoke_fee_mint(*mint_key);
    Ok(())
}

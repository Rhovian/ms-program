use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::{errors::MadscapeError, state::ReleaseAuthority, constants::RELEASE};

#[derive(Accounts)]
pub struct ApproveFeeMint<'info> {
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

pub fn approve_fee_mint(ctx: Context<ApproveFeeMint>, fee: u64) -> Result<()> {
    let release_authority = &mut ctx.accounts.release_authority;
    let mint_key = &ctx.accounts.mint.key();
    require!(
        !release_authority
            .approved_mints
            .iter()
            .any(|i| &i.mint == mint_key),
        MadscapeError::FeeMintAlreadyApproved
    );
    release_authority.approve_fee_mint(*mint_key, fee);
    Ok(())
}

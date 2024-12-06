use anchor_lang::prelude::*;

use crate::{constants::RELEASE, state::ReleaseAuthority};

#[derive(Accounts)]
pub struct UpdateReleaseAuthority<'info> {
    #[account(
        mut,
        seeds = [RELEASE.as_ref(), authority.key().as_ref()], 
        bump = release_authority.bump,
    )]
    pub release_authority: Account<'info, ReleaseAuthority>,
    /// CHECK: Checked by design logic.
    pub treasury: AccountInfo<'info>,
    #[account(address = release_authority.authority)]
    pub authority: Signer<'info>,
}

pub fn update_release_authority(ctx: Context<UpdateReleaseAuthority>, fee_lamports_basis_points: u16) -> Result<()> {
    let release_authority = &mut ctx.accounts.release_authority;
    let treasury = ctx.accounts.treasury.key();
    release_authority.update(treasury, fee_lamports_basis_points);
    Ok(())
}

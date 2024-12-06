use anchor_lang::prelude::*;

use crate::{constants::RELEASE, state::ReleaseAuthority};

#[derive(Accounts)]
pub struct CreateReleaseAuthority<'info> {
    #[account(
        init,
        payer = authority,
        seeds = [RELEASE.as_ref(), authority.key().as_ref()], 
        space = ReleaseAuthority::space(),
        bump,
    )]
    pub release_authority: Account<'info, ReleaseAuthority>,
    /// CHECK: Checked by design logic.
    pub treasury: AccountInfo<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_release_authority(ctx: Context<CreateReleaseAuthority>, fee_lamports_basis_points: u16) -> Result<()> {
    let release_authority = &mut ctx.accounts.release_authority;
    let bump = ctx.bumps.release_authority;
    let authority = ctx.accounts.authority.key();
    let treasury = ctx.accounts.treasury.key();
    release_authority.set_inner(ReleaseAuthority::new(bump, authority, treasury, fee_lamports_basis_points));
    Ok(())
}

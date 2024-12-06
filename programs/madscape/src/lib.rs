#![allow(clippy::result_large_err)]
#![allow(clippy::too_many_arguments)]
pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod strings;
pub mod maths;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("moonKi1FYsaWQnmqjvnLTXZbDkwpEzzCEXtnfo2bJHC");

#[program]
pub mod madscape {
    use super::*;

    // Admin IXs
    pub fn create_release_authority(ctx: Context<CreateReleaseAuthority>, fee: u16) -> Result<()> {
        instructions::create_release_authority(ctx, fee)
    }
    
    pub fn approve_fee_mint(ctx: Context<ApproveFeeMint>, fee: u64) -> Result<()> {
        instructions::approve_fee_mint(ctx, fee)
    }

    pub fn revoke_fee_mint(ctx: Context<RevokeFeeMint>) -> Result<()> {
        instructions::revoke_fee_mint(ctx)
    }

    pub fn update_release_authority(ctx: Context<UpdateReleaseAuthority>, fee_lamports_basis_points: u16) -> Result<()> {
        instructions::update_release_authority(ctx, fee_lamports_basis_points)
    }

    // Match IXs SOL
    pub fn create_open_match(ctx: Context<CreateOpenMatch>, amount: u64, match_type: u8) -> Result<()> {
        instructions::sol::create_open_match(ctx, amount, match_type)
    }

    pub fn create_private_match(ctx: Context<CreatePrivateMatch>, amount: u64, match_type: u8, user_b: Pubkey) -> Result<()> {
        instructions::sol::create_private_match(ctx, amount, match_type, user_b)
    }

    pub fn join_match(ctx: Context<JoinMatch>) -> Result<()> {
        instructions::sol::join_match(ctx)
    }

    pub fn cancel_open_match(ctx: Context<CancelOpenMatch>) -> Result<()> {
        instructions::sol::cancel_open_match(ctx)
    }

    pub fn cancel_private_match(ctx: Context<CancelPrivateMatch>) -> Result<()> {
        instructions::sol::cancel_private_match(ctx)
    }

    pub fn end_match(ctx: Context<EndMatch>, winner: Pubkey) -> Result<()> {
        instructions::sol::end_match(ctx, winner)
    }

    // Match IXs SOL
    pub fn create_open_match_mint(ctx: Context<CreateOpenMatchMint>, amount: u64, match_type: u8) -> Result<()> {
        instructions::create_open_match_mint(ctx, amount, match_type)
    }

    pub fn create_private_match_mint(ctx: Context<CreatePrivateMatchMint>, amount: u64, match_type: u8, user_b: Pubkey) -> Result<()> {
        instructions::create_private_match_mint(ctx, amount, match_type, user_b)
    }

    pub fn join_match_mint(ctx: Context<JoinMatchMint>) -> Result<()> {
        instructions::join_match_mint(ctx)
    }

    pub fn cancel_open_match_mint(ctx: Context<CancelOpenMatchMint>) -> Result<()> {
        instructions::cancel_open_match_mint(ctx)
    }

    pub fn cancel_private_match_mint(ctx: Context<CancelPrivateMatchMint>) -> Result<()> {
        instructions::cancel_private_match_mint(ctx)
    }

    pub fn end_match_mint(ctx: Context<EndMatchMint>) -> Result<()> {
        instructions::end_match_mint(ctx)
    }

    pub fn close_match(ctx: Context<CloseMatch>) -> Result<()> {
        instructions::close_match(ctx)
    }
}

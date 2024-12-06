use anchor_lang::prelude::*;

use crate::constants::MAX_ALLOWED_APPROVED_MINTS;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default, PartialEq)]
pub struct ReleaseAuthorityItem {
    pub mint: Pubkey,
    pub fee: u64,
}

impl ReleaseAuthorityItem {
    pub fn new(mint: Pubkey, fee: u64) -> Self {
        Self { mint, fee }
    }
    pub fn space() -> usize {
        // No discriminator since this is a nested struct
        32 + // mint
        8 // fee
    }
}
#[account]
pub struct ReleaseAuthority {
    pub bump: u8,
    pub authority: Pubkey,
    pub treasury: Pubkey,
    pub approved_mints: Vec<ReleaseAuthorityItem>,
    pub fee_lamports_basis_points: u16,
}

impl ReleaseAuthority {
    pub fn new(bump: u8, authority: Pubkey, treasury: Pubkey, fee_lamports_basis_points: u16) -> Self {
        Self {
            bump,
            authority,
            approved_mints: vec![],
            treasury,
            fee_lamports_basis_points,
        }
    }

    pub fn update(&mut self, treasury: Pubkey, fee_lamports_basis_points: u16) {
        self.treasury = treasury;
        self.fee_lamports_basis_points = fee_lamports_basis_points;
    }

    pub fn approve_fee_mint(&mut self, mint: Pubkey, fee: u64) {
        self.approved_mints
            .push(ReleaseAuthorityItem::new(mint, fee));
    }

    pub fn revoke_fee_mint(&mut self, mint: Pubkey) {
        self.approved_mints.retain(|i| i.mint != mint);
    }

    pub fn space() -> usize {
        8 + // Discriminator
        1 + // bump
        32 + // authority
        32 + // treasury
        4 + ReleaseAuthorityItem::space() * MAX_ALLOWED_APPROVED_MINTS + // approved_mints
        2 // fee
    }
}

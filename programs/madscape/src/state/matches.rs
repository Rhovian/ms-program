use crate::constants::{SOLANA_PUBKEY, MAX_MATCH_ID_LENGTH};
use anchor_lang::prelude::*;

pub fn is_valid_match_type_for_init(match_type: u8) -> bool {
    matches!(match_type, 1..=8)
}

#[account]
pub struct Match {
    pub target_mint: Pubkey,
    pub user_a: Pubkey,
    pub user_b: Pubkey,
    pub target_amount: u64,
    pub init_timestamp: i64,
    pub join_timestamp: i64,
    pub active: bool,
    pub match_type: u8,
    pub match_id: String,
    pub mint: Pubkey,
    pub completed: bool,
    pub server: u8,
    pub bump: u8,
    pub release_authority: Pubkey,
    pub fee_amount: u64,
}

impl Match {
    pub fn new(
        bump: u8,
        release_authority: Pubkey,
        user_a: Pubkey,
        mint: Pubkey,  
        created_at: i64, 
    ) -> Self {
        
        Self {
            bump,
            release_authority, 
            target_mint: Pubkey::default(),
            user_a,
            user_b: Pubkey::default(),
            target_amount: 0,
            init_timestamp: created_at,
            join_timestamp: 0,
            active: false,
            match_type: 0,
            match_id: "".to_string(),
            mint,
            completed: false,
            fee_amount: 0,
            server: 0,
        }
    }

    pub fn new_private(
        bump: u8,
        release_authority: Pubkey,
        user_a: Pubkey,
        user_b: Pubkey,
        mint: Pubkey,  
        created_at: i64,
    ) -> Self {
        
        Self {
            bump,
            release_authority,
            target_mint: Pubkey::default(),
            user_a,
            user_b,
            target_amount: 0,
            init_timestamp: created_at,
            join_timestamp: 0,
            active: false,
            match_type: 0,
            match_id: "".to_string(),
            mint, 
            completed: false,
            fee_amount: 0,
            server: 0,
        }
    }

    pub fn is_not_initialized(&self) -> bool {
        self.fee_amount == 0
            && self.target_mint == Pubkey::default()
            && self.target_amount == 0
            && !self.active
            && self.match_type == 0
            && self.match_id.is_empty()
    }

    pub fn is_initialized(&self) -> bool {
        !self.is_not_initialized() 
    }

    pub fn init(
        &mut self,
        fee_amount: u64,
        target_mint: Pubkey,
        target_amount: u64,
        match_type: u8,
        match_id: String,
        mint: Pubkey,
    ) {
        self.fee_amount = fee_amount;
        self.target_mint = target_mint;
        self.target_amount = target_amount;
        self.match_type = match_type;
        self.match_id = match_id;
        self.mint = mint;  
        self.server = (Clock::get().unwrap().unix_timestamp % 10) as u8;
    }

    pub fn is_native_sol(&self) -> bool {
        self.target_mint == SOLANA_PUBKEY
    }

    pub fn activate(&mut self, joined_at: i64) {
        self.active = true;
        self.join_timestamp = joined_at;
    }

    pub fn activate_public(&mut self, user_b: Pubkey, joined_at: i64) {
        self.active = true;
        self.user_b = user_b;
        self.join_timestamp = joined_at;
    }

    pub fn retire(&mut self) {
        self.completed = true;
    }
    pub fn space() -> usize {
        8 + // Discriminator
        1 + // bump  
        32 + // release_authority
        32 + // target_mint
        32 + // user_a
        32 + // user_b
        8 + // target_amount
        8 + // init_timestamp 
        8 + // join_timestamp
        1 + // active
        1 + // match_type
        4 + MAX_MATCH_ID_LENGTH + // match_id
        32 + // mint
        1 + // completed
        1 + // server
        8 // fee_amount
    }
}

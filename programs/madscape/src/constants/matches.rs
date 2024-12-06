use anchor_lang::{solana_program::pubkey};
use solana_program::pubkey::Pubkey;

pub const MATCH: [u8; 5] = *b"match";
pub const MATCH_TOKEN_ACCOUNT: [u8; 19] = *b"match_token_account";
pub static SOLANA_PUBKEY: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
pub const MAX_MATCH_ID_LENGTH: usize = 64;
pub const USER_A: [u8; 6] = *b"user_a";
pub const USER_B: [u8; 6] = *b"user_b";
pub const USER_A_TOKEN_ACCOUNT: [u8; 20] = *b"user_a_token_account";
pub const USER_B_TOKEN_ACCOUNT: [u8; 20] = *b"user_b_token_account";


use arrayref::array_ref;
use solana_program::{account_info::AccountInfo, clock::Clock, pubkey::Pubkey};

use crate::constants::{MATCH, MAX_MATCH_ID_LENGTH};

/// Pads a string with a character to a maximum length.
/// 
/// # Arguments
/// 
/// * `string` - The string to pad.
/// * `max_length` - The maximum length of the string.
/// * `character` - The character to pad the string with.
/// 
/// # Returns
/// 
/// * `String` - The padded string.
/// 
/// # Example
/// 
/// ```
/// use madscape::tools::strings::pad_str;
/// 
/// let padded_string = pad_str("Hello", 10, '_');
/// assert_eq!(padded_string, "Hello_____");
/// ```
pub fn pad_str(string: &str, max_length: usize, character: char) -> String {
    let mut padded_string = string.to_string();
    let mut padding = String::new();
    for _ in 0..(max_length - string.len()) {
        padding.push(character);
    }
    padded_string.push_str(&padding);
    padded_string
}

/// Generates an escrow id from the most recent slothash and the current unix timestamp.
/// 
/// # Arguments
/// 
/// * `recent_slothashes` - The account info for the recent slothashes account.
/// * `clock` - The clock struct.
/// 
/// # Returns
/// 
/// * `String` - The escrow id following the format `0x{generated_game_key}+_______`.
pub fn generate_escrow_id(recent_slothashes: &AccountInfo<'_>, clock: Clock) -> String {
    let slothash_data = recent_slothashes.data.borrow();
    let most_recent_slothash_bytes = array_ref![slothash_data, 12, 8];
    let clock_bytes = clock.unix_timestamp.to_le_bytes();

    // Although this is a pda address, it really is a random address
    // generated from the slothash + clock and the program id.
    // (Should be random enough).
    let (generated_game_key, _) = Pubkey::find_program_address(
        &[
            MATCH.as_ref(),
            most_recent_slothash_bytes,
            clock_bytes.as_ref(),
        ],
        &crate::ID,
    );
    let escrow_id = format!("{}", generated_game_key);
    pad_str(escrow_id.as_str(), MAX_MATCH_ID_LENGTH, '_')
}

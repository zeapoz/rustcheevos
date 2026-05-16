//! Utility functions.

use std::num::ParseIntError;

/// Parses a hex address string with optional `0x` prefix.
///
/// # Errors
///
/// Returns a [`ParseIntError`] if the string contains invalid hex characters.
pub fn parse_hex_address(s: &str) -> Result<usize, ParseIntError> {
    let s = s.trim().strip_prefix("0x").unwrap_or(s);
    usize::from_str_radix(s, 16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_address_valid() {
        assert_eq!(parse_hex_address("0x1234").unwrap(), 0x1234);
        assert_eq!(parse_hex_address("1234").unwrap(), 0x1234);
        assert_eq!(parse_hex_address("  0xABCD  ").unwrap(), 0xABCD);
    }

    #[test]
    fn test_parse_hex_address_invalid() {
        assert!(parse_hex_address("xyz").is_err());
        assert!(parse_hex_address("").is_err());
    }
}

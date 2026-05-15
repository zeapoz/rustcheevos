//! Parser functions for memory types.

use winnow::{
    Parser, Result,
    ascii::hex_digit1,
    combinator::{alt, opt},
    token::one_of,
};

use crate::prelude::{AccessMode, MemoryRef, MemorySize};

/// Parses a memory access mode.
pub fn parse_memory_access_mode(input: &mut &str) -> Result<AccessMode> {
    one_of(['d', 'p', 'b', '~'])
        .try_map(AccessMode::try_from)
        .parse_next(input)
}

/// Parses a memory size.
pub fn parse_memory_size(input: &mut &str) -> Result<MemorySize> {
    let bits = one_of([
        'H', ' ', 'X', 'W', 'I', 'J', 'G', 'K', 'L', 'U', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
    ]);
    let floats = one_of(['F', 'B', 'H', 'I', 'M', 'L']);

    let (_prefix, memsize) = alt((
        ("0x", bits.try_map(MemorySize::parse_bit_size)),
        ("f", floats.try_map(MemorySize::parse_float_size)),
    ))
    .parse_next(input)?;

    Ok(memsize)
}

/// Parses a memory reference.
pub fn parse_memory_ref(input: &mut &str) -> Result<MemoryRef> {
    let (access_mode, memsize, addr) = (
        opt(parse_memory_access_mode),
        parse_memory_size,
        parse_hex_address,
    )
        .parse_next(input)?;
    Ok(MemoryRef::new(memsize, addr).with_access_mode(access_mode.unwrap_or_default()))
}

/// Parses a hex address.
fn parse_hex_address(input: &mut &str) -> Result<usize> {
    hex_digit1
        .try_map(|hex| usize::from_str_radix(hex, 16))
        .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_memory_size() {
        let input = "0xH";
        let memory_size = input.parse::<MemorySize>().unwrap();
        assert_eq!(memory_size, MemorySize::Bits8);
    }

    #[test]
    fn test_parse_no_memory_size() {
        let input = "";
        let memory_size_or_err = input.parse::<MemorySize>();
        assert!(memory_size_or_err.is_err());
    }

    #[test]
    fn test_parse_invalid_memory_size() {
        let input = "e";
        let memory_size_or_err = input.parse::<MemorySize>();
        assert!(memory_size_or_err.is_err());
    }

    #[test]
    fn test_parse_valid_bit_memory_ref() {
        let input = "0xH1234";
        let memory_ref = input.parse::<MemoryRef>().unwrap();
        assert_eq!(memory_ref.size(), MemorySize::Bits8);
        assert_eq!(memory_ref.address(), 0x1234);
        assert_eq!(memory_ref.access_mode(), AccessMode::default());
    }

    #[test]
    fn test_parse_valid_float_memory_ref() {
        let input = "fF1234";
        let memory_ref = input.parse::<MemoryRef>().unwrap();
        assert_eq!(memory_ref.size(), MemorySize::Float);
        assert_eq!(memory_ref.address(), 0x1234);
        assert_eq!(memory_ref.access_mode(), AccessMode::default());
    }

    #[test]
    fn test_parse_no_memory_ref() {
        let input = "1234";
        let memory_ref_or_err = input.parse::<MemoryRef>();
        assert!(memory_ref_or_err.is_err());
    }

    #[test]
    fn test_parse_invalid_memory_ref() {
        let input = "0xHtest";
        let memory_ref_or_err = input.parse::<MemoryRef>();
        assert!(memory_ref_or_err.is_err());
    }
}

//! Parser functions for memory types.

use winnow::{Parser, Result, token::one_of};

use crate::types::flag::{ArithmeticFlag, ConditionFlag};

/// Parses a comparison flag.
pub fn parse_condition_flag(input: &mut &str) -> Result<ConditionFlag> {
    let flags = one_of(['P', 'R', 'Z', 'C', 'D', 'N', 'O', 'M', 'G', 'Q', 'T']);

    let flag = flags.try_map(ConditionFlag::try_from).parse_next(input)?;
    let _colon = ":".parse_next(input)?;
    Ok(flag)
}

/// Parses an arithmetic flag.
pub fn parse_arithmetic_flag(input: &mut &str) -> Result<ArithmeticFlag> {
    let flags = one_of(['A', 'B', 'I', 'K', 'M']);

    let flag = flags.try_map(ArithmeticFlag::try_from).parse_next(input)?;
    let _colon = ":".parse_next(input)?;
    Ok(flag)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_condition_flag() {
        let mut input = "P:";
        let flag = parse_condition_flag(&mut input).unwrap();
        assert_eq!(flag, ConditionFlag::PauseIf);
    }

    #[test]
    fn test_parse_valid_arithmetic_flag() {
        let mut input = "A:";
        let flag = parse_arithmetic_flag(&mut input).unwrap();
        assert_eq!(flag, ArithmeticFlag::AddSource);
    }

    #[test]
    fn test_parse_no_condition_flag() {
        let mut input = "";
        assert!(parse_condition_flag(&mut input).is_err());
    }

    #[test]
    fn test_parse_no_arithmetic_flag() {
        let mut input = "";
        assert!(parse_arithmetic_flag(&mut input).is_err());
    }

    #[test]
    fn test_parse_invalid_condition_flag() {
        let mut input = "E:";
        assert!(parse_condition_flag(&mut input).is_err());
    }

    #[test]
    fn test_parse_invalid_arithmetic_flag() {
        let mut input = "E:";
        assert!(parse_arithmetic_flag(&mut input).is_err());
    }

    #[test]
    fn test_parse_condition_flag_no_colon() {
        let mut input = "P ";
        assert!(parse_condition_flag(&mut input).is_err());
    }

    #[test]
    fn test_parse_arithmetic_flag_no_colon() {
        let mut input = "A ";
        assert!(parse_arithmetic_flag(&mut input).is_err());
    }
}

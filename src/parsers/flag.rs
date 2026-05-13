use winnow::{Parser, Result, combinator::alt, token::one_of};

use crate::types::flag::*;

pub fn parse_comparison_flag(input: &mut &str) -> Result<ComparisonFlag> {
    let flags = one_of(['P', 'R', 'Z', 'C', 'D', 'N', 'O', 'M', 'G', 'Q', 'T']);

    let flag = flags.try_map(ComparisonFlag::try_from).parse_next(input)?;
    let _colon = ":".parse_next(input)?;
    Ok(flag)
}

pub fn parse_arithmetic_flag(input: &mut &str) -> Result<ArithmeticFlag> {
    let flags = one_of(['A', 'B', 'I', 'K', 'M']);

    let flag = flags.try_map(ArithmeticFlag::try_from).parse_next(input)?;
    let _colon = ":".parse_next(input)?;
    Ok(flag)
}

pub fn parse_flag(input: &mut &str) -> Result<Flag> {
    let flag = alt((
        parse_comparison_flag.map(Flag::from),
        parse_arithmetic_flag.map(Flag::from),
    ))
    .parse_next(input)?;
    Ok(flag)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_flag() {
        let input = "A:";
        let flag = input.parse::<Flag>().unwrap();
        assert_eq!(flag, Flag::Arithmetic(ArithmeticFlag::AddSource));
    }

    #[test]
    fn test_parse_no_flag() {
        let input = "";
        let flag_or_err = input.parse::<Flag>();
        assert!(flag_or_err.is_err());
    }

    #[test]
    fn test_parse_invalid_flag() {
        let input = "E";
        let flag_or_err = input.parse::<Flag>();
        assert!(flag_or_err.is_err());
    }

    #[test]
    fn test_parse_valid_flag_no_colon() {
        let input = "A ";
        let flag_or_err = input.parse::<Flag>();
        assert!(flag_or_err.is_err());
    }
}

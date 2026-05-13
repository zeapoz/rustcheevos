use winnow::{Parser, Result, combinator::alt};

use crate::types::operator::*;

pub fn parse_arithmetic_operator(input: &mut &str) -> Result<ArithmeticOperator> {
    let operators = alt(("+", "-", "*", "/"));
    operators
        .try_map(ArithmeticOperator::try_from)
        .parse_next(input)
}

pub fn parse_comparison_operator(input: &mut &str) -> Result<ComparisonOperator> {
    let operators = alt(("<=", "<", ">=", ">", "!=", "="));
    operators
        .try_map(ComparisonOperator::try_from)
        .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_operator() {
        let input = "+";
        let operator = input.parse::<ArithmeticOperator>().unwrap();
        assert_eq!(operator, ArithmeticOperator::Add);
    }

    #[test]
    fn test_parse_no_operator() {
        let input = "";
        let operator_or_err = input.parse::<ComparisonOperator>();
        assert!(operator_or_err.is_err());
    }

    #[test]
    fn test_parse_invalid_operator() {
        let input = "E";
        let operator_or_err = input.parse::<ArithmeticOperator>();
        assert!(operator_or_err.is_err());
    }

    #[test]
    fn test_parse_valid_comparison_operator() {
        let input = "<=";
        let operator = input.parse::<ComparisonOperator>().unwrap();
        assert_eq!(operator, ComparisonOperator::LessThanOrEquals);
    }
}

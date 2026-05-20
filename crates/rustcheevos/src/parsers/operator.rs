//! Parser functions for operators.

use winnow::{Parser, Result, combinator::alt};

use crate::types::operator::{ArithmeticOperator, ConditionOperator};

/// Parses an arithmetic operator.
pub fn parse_arithmetic_operator(input: &mut &str) -> Result<ArithmeticOperator> {
    let operators = alt(("+", "-", "*", "/", "%", "&", "^"));
    operators
        .try_map(ArithmeticOperator::try_from)
        .parse_next(input)
}

/// Parses a comparison operator.
pub fn parse_condition_operator(input: &mut &str) -> Result<ConditionOperator> {
    let operators = alt(("<=", "<", ">=", ">", "!=", "="));
    operators
        .try_map(ConditionOperator::try_from)
        .parse_next(input)
}

#[cfg(test)]
mod tests {
    use crate::types::operator::ArithmeticOperator;

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
        let operator_or_err = input.parse::<ConditionOperator>();
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
        let operator = input.parse::<ConditionOperator>().unwrap();
        assert_eq!(operator, ConditionOperator::LessThanOrEquals);
    }

    #[test]
    fn test_parse_modulo_operator() {
        let input = "%";
        let operator = input.parse::<ArithmeticOperator>().unwrap();
        assert_eq!(operator, ArithmeticOperator::Modulo);
    }

    #[test]
    fn test_parse_bitwise_and_operator() {
        let input = "&";
        let operator = input.parse::<ArithmeticOperator>().unwrap();
        assert_eq!(operator, ArithmeticOperator::BitwiseAnd);
    }

    #[test]
    fn test_parse_bitwise_xor_operator() {
        let input = "^";
        let operator = input.parse::<ArithmeticOperator>().unwrap();
        assert_eq!(operator, ArithmeticOperator::BitwiseXor);
    }
}

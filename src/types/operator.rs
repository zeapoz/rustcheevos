use std::{fmt, str::FromStr};

use winnow::Parser;

use crate::{
    ParseError,
    parsers::{parse_arithmetic_operator, parse_comparison_operator},
};

/// Operators that can be used in arithmetic.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ArithmeticOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    BitwiseAnd,
    BitwiseXor,
}

impl TryFrom<&str> for ArithmeticOperator {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "+" => Ok(ArithmeticOperator::Add),
            "-" => Ok(ArithmeticOperator::Subtract),
            "*" => Ok(ArithmeticOperator::Multiply),
            "/" => Ok(ArithmeticOperator::Divide),
            "%" => Ok(ArithmeticOperator::Modulo),
            "&" => Ok(ArithmeticOperator::BitwiseAnd),
            "^" => Ok(ArithmeticOperator::BitwiseXor),
            _ => Err(ParseError::InvalidOperator(s.to_string())),
        }
    }
}

impl FromStr for ArithmeticOperator {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_arithmetic_operator
            .parse(s)
            .map_err(|s| ParseError::InvalidOperator(s.to_string()))
    }
}

impl fmt::Display for ArithmeticOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ArithmeticOperator::Add => "+",
            ArithmeticOperator::Subtract => "-",
            ArithmeticOperator::Multiply => "*",
            ArithmeticOperator::Divide => "/",
            ArithmeticOperator::Modulo => "%",
            ArithmeticOperator::BitwiseAnd => "&",
            ArithmeticOperator::BitwiseXor => "^",
        };
        write!(f, "{s}")
    }
}

/// Operators that can be used in comparisons.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ComparisonOperator {
    LessThan,
    LessThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,
    Equals,
    NotEquals,
}

impl TryFrom<&str> for ComparisonOperator {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "<" => Ok(ComparisonOperator::LessThan),
            "<=" => Ok(ComparisonOperator::LessThanOrEquals),
            ">" => Ok(ComparisonOperator::GreaterThan),
            ">=" => Ok(ComparisonOperator::GreaterThanOrEquals),
            "=" => Ok(ComparisonOperator::Equals),
            "!=" => Ok(ComparisonOperator::NotEquals),
            _ => Err(ParseError::InvalidOperator(s.to_string())),
        }
    }
}

impl FromStr for ComparisonOperator {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_comparison_operator
            .parse(s)
            .map_err(|s| ParseError::InvalidOperator(s.to_string()))
    }
}

impl fmt::Display for ComparisonOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ComparisonOperator::LessThan => "<",
            ComparisonOperator::LessThanOrEquals => "<=",
            ComparisonOperator::GreaterThan => ">",
            ComparisonOperator::GreaterThanOrEquals => ">=",
            ComparisonOperator::Equals => "=",
            ComparisonOperator::NotEquals => "!=",
        };
        write!(f, "{s}")
    }
}

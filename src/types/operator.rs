//! Comparison operators for conditions.

use crate::types::ParseError;
use std::str::FromStr;

/// Comparison operators for conditions.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    /// Equality comparison.
    Equals,
    /// Inequality comparison.
    NotEquals,
    /// Less than comparison.
    LessThan,
    /// Less than or equal comparison.
    LessThanOrEquals,
    /// Greater than comparison.
    GreaterThan,
    /// Greater than or equal comparison.
    GreaterThanOrEquals,
    /// Addition operation.
    Add,
    /// Subtraction operation.
    Subtract,
    /// Multiplication operation.
    Multiply,
    /// Division operation.
    Divide,
}

impl Operator {
    /// Returns the string prefix for this operator.
    ///
    /// # Returns
    ///
    /// The string prefix.
    pub fn to_prefix(&self) -> &'static str {
        match self {
            Operator::Equals => "=",
            Operator::NotEquals => "!=",
            Operator::LessThan => "<",
            Operator::LessThanOrEquals => "<=",
            Operator::GreaterThan => ">",
            Operator::GreaterThanOrEquals => ">=",
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
        }
    }
}

impl FromStr for Operator {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Operator::Multiply),
            "/" => Ok(Operator::Divide),
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Subtract),
            "=" => Ok(Operator::Equals),
            ">=" => Ok(Operator::GreaterThanOrEquals),
            "<=" => Ok(Operator::LessThanOrEquals),
            ">" => Ok(Operator::GreaterThan),
            "<" => Ok(Operator::LessThan),
            "!=" => Ok(Operator::NotEquals),
            _ => Err(ParseError::UnknownOperator(s.to_string())),
        }
    }
}

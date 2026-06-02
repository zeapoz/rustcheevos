//! Type definitions for operators.

use std::{fmt, str::FromStr};

use winnow::Parser;

use crate::parsers::{ParseError, parse_arithmetic_operator, parse_condition_operator};

/// A container for all operators.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    /// An operator used in comparisons.
    Condition(ConditionOperator),
    /// An operator used in arithmetic operations.
    Arithmetic(ArithmeticOperator),
}

impl From<ConditionOperator> for Operator {
    fn from(operator: ConditionOperator) -> Self {
        Self::Condition(operator)
    }
}

impl From<ArithmeticOperator> for Operator {
    fn from(operator: ArithmeticOperator) -> Self {
        Self::Arithmetic(operator)
    }
}

/// Operators that can be used in arithmetic.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ArithmeticOperator {
    /// The addition operator.
    Add,
    /// The subtraction operator.
    Subtract,
    /// The multiplication operator.
    Multiply,
    /// The division operator.
    Divide,
    /// The modulo operator.
    Modulo,
    /// The bitwise and operator.
    BitwiseAnd,
    /// The bitwise xor operator.
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
            _ => Err(ParseError::Operator(s.to_string())),
        }
    }
}

impl FromStr for ArithmeticOperator {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_arithmetic_operator
            .parse(s)
            .map_err(|s| ParseError::Operator(s.to_string()))
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
pub enum ConditionOperator {
    /// The less than operator.
    LessThan,
    /// The less than or equals operator.
    LessThanOrEquals,
    /// The greater than operator.
    GreaterThan,
    /// The greater than or equals operator.
    GreaterThanOrEquals,
    /// The equals operator.
    Equals,
    /// The not equals operator.
    NotEquals,
}

impl TryFrom<&str> for ConditionOperator {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "<" => Ok(ConditionOperator::LessThan),
            "<=" => Ok(ConditionOperator::LessThanOrEquals),
            ">" => Ok(ConditionOperator::GreaterThan),
            ">=" => Ok(ConditionOperator::GreaterThanOrEquals),
            "=" => Ok(ConditionOperator::Equals),
            "!=" => Ok(ConditionOperator::NotEquals),
            _ => Err(ParseError::Operator(s.to_string())),
        }
    }
}

impl FromStr for ConditionOperator {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_condition_operator
            .parse(s)
            .map_err(|s| ParseError::Operator(s.to_string()))
    }
}

impl fmt::Display for ConditionOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ConditionOperator::LessThan => "<",
            ConditionOperator::LessThanOrEquals => "<=",
            ConditionOperator::GreaterThan => ">",
            ConditionOperator::GreaterThanOrEquals => ">=",
            ConditionOperator::Equals => "=",
            ConditionOperator::NotEquals => "!=",
        };
        write!(f, "{s}")
    }
}

use crate::types::ParseError;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    Equals,
    NotEquals,
    LessThan,
    LessThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,
    Add,
    Subtract,
    Multiply,
    Divide,
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

impl Operator {
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

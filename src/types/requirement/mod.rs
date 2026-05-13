use std::{fmt, str::FromStr};

use winnow::Parser;

use crate::{parsers::ParseError, parsers::parse_requirement};

use arithmetic::ArithmeticRequirement;
use comparison::ComparisonRequirement;

pub mod arithmetic;
pub mod comparison;

/// A single requirement clause.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Requirement {
    /// A comparison between two values.
    Comparison(ComparisonRequirement),
    /// An arithmetic operation between two values.
    Arithmetic(ArithmeticRequirement),
}

impl Requirement {
    /// Returns a requirement that always evaluates to true.
    pub fn always_true() -> Self {
        Requirement::Comparison(ComparisonRequirement::eq(0, 0))
    }

    /// Returns a requirement that always evaluates to false.
    pub fn always_false() -> Self {
        Requirement::Comparison(ComparisonRequirement::eq(0, 1))
    }
}

impl From<ComparisonRequirement> for Requirement {
    fn from(requirement: ComparisonRequirement) -> Self {
        Requirement::Comparison(requirement)
    }
}

impl From<ArithmeticRequirement> for Requirement {
    fn from(requirement: ArithmeticRequirement) -> Self {
        Requirement::Arithmetic(requirement)
    }
}

impl FromStr for Requirement {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_requirement
            .parse(s)
            .map_err(|s| ParseError::InvalidRequirement(s.to_string()))
    }
}

impl fmt::Display for Requirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Requirement::Comparison(requirement) => write!(f, "{requirement}"),
            Requirement::Arithmetic(requirement) => write!(f, "{requirement}"),
        }
    }
}

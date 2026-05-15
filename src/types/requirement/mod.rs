//! Type definitions for requirements.

use std::{fmt, str::FromStr};

use winnow::Parser;

use crate::{
    prelude::AccessMode,
    types::memory::AccessModeModifier,
    parsers::ParseError,
    parsers::parse_requirement,
};

pub mod arithmetic;
pub mod condition;

pub use arithmetic::Arithmetic;
pub use condition::Condition;

/// A single requirement clause.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Requirement {
    /// A comparison between two values.
    Condition(Condition),
    /// An arithmetic operation between two values.
    Arithmetic(Arithmetic),
}

impl From<Condition> for Requirement {
    fn from(requirement: Condition) -> Self {
        Requirement::Condition(requirement)
    }
}

impl From<Arithmetic> for Requirement {
    fn from(requirement: Arithmetic) -> Self {
        Requirement::Arithmetic(requirement)
    }
}

impl FromStr for Requirement {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_requirement
            .parse(s)
            .map_err(|s| ParseError::Condition(s.to_string()))
    }
}

impl fmt::Display for Requirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Requirement::Condition(requirement) => write!(f, "{requirement}"),
            Requirement::Arithmetic(requirement) => write!(f, "{requirement}"),
        }
    }
}

impl AccessModeModifier for Requirement {
    fn with_access_mode(mut self, access_mode: AccessMode) -> Self {
        self = match self {
            Requirement::Condition(c) => Requirement::Condition(c.with_access_mode(access_mode)),
            Requirement::Arithmetic(a) => Requirement::Arithmetic(a.with_access_mode(access_mode)),
        };
        self
    }
}

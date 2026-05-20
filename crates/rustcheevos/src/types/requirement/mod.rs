//! Type definitions for requirements.

use std::{fmt, str::FromStr};

use winnow::Parser;

use crate::{
    impl_arithmetic_flag_traits, impl_condition_flag_traits,
    parsers::ParseError,
    parsers::parse_requirement,
    types::{
        flag::{ArithmeticFlag, ConditionFlag},
        memory::{AccessMode, AccessModeModifier},
    },
};

pub(crate) mod arithmetic;
pub(crate) mod condition;

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

impl Requirement {
    /// Sets the given comparison flag on this requirement if it is a [`Condition`].
    ///
    /// If the requirement is an [`Arithmetic`], returns self unchanged.
    #[must_use]
    pub fn with_condition_flag(self, flag: ConditionFlag) -> Self {
        match self {
            Requirement::Condition(c) => Requirement::Condition(c.with_flag(flag)),
            Requirement::Arithmetic(a) => Requirement::Arithmetic(a),
        }
    }

    /// Sets the given arithmetic flag on this requirement if it is an [`Arithmetic`].
    ///
    /// If the requirement is a [`Condition`], returns self unchanged.
    #[must_use]
    pub fn with_arithmetic_flag(self, flag: ArithmeticFlag) -> Self {
        match self {
            Requirement::Condition(c) => Requirement::Condition(c),
            Requirement::Arithmetic(a) => Requirement::Arithmetic(a.with_flag(flag)),
        }
    }
}

impl_condition_flag_traits!(Requirement, with_condition_flag);
impl_arithmetic_flag_traits!(Requirement, with_arithmetic_flag);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_requirement_condition() {
        let original: Requirement = "0xH1234=50".parse().unwrap();
        let serialized = original.to_string();
        let parsed: Requirement = serialized.parse().unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn roundtrip_requirement_arithmetic() {
        let original: Requirement = "A:0xH1234+50".parse().unwrap();
        let serialized = original.to_string();
        let parsed: Requirement = serialized.parse().unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn roundtrip_requirement_with_hit_count() {
        let original: Requirement = "0xH1234>=10.5.".parse().unwrap();
        let serialized = original.to_string();
        let parsed: Requirement = serialized.parse().unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn roundtrip_requirement_memory_comparison() {
        let original: Requirement = "d0xX1234<0xX5678".parse().unwrap();
        let serialized = original.to_string();
        let parsed: Requirement = serialized.parse().unwrap();
        assert_eq!(original, parsed);
    }
}

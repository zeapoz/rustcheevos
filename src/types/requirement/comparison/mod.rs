use hits::HitCount;
use std::{fmt, str::FromStr};
use winnow::Parser;

pub mod hits;

use crate::{
    impl_comparison_flag_traits,
    parsers::ParseError,
    parsers::parse_comparison_requirement,
    types::{flag::ComparisonFlag, operator::ComparisonOperator, value::TypedValue},
};

/// A comparison between two values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComparisonRequirement {
    pub flag: Option<ComparisonFlag>,
    pub lhs: TypedValue,
    pub operation: ComparisonOperation,
    pub hit_count: HitCount,
}

impl ComparisonRequirement {
    /// Returns a new comparison between two values.
    pub fn new(
        lhs: impl Into<TypedValue>,
        comparison: ComparisonOperator,
        rhs: impl Into<TypedValue>,
    ) -> Self {
        Self {
            flag: None,
            lhs: lhs.into(),
            operation: ComparisonOperation {
                comparator: comparison,
                rhs: rhs.into(),
            },
            hit_count: HitCount::default(),
        }
    }

    /// Returns a new equals comparison between two values.
    pub fn eq(lhs: impl Into<TypedValue>, rhs: impl Into<TypedValue>) -> Self {
        Self::new(lhs, ComparisonOperator::Equals, rhs)
    }

    /// Returns a new not equals comparison between two values.
    pub fn ne(lhs: impl Into<TypedValue>, rhs: impl Into<TypedValue>) -> Self {
        Self::new(lhs, ComparisonOperator::NotEquals, rhs)
    }

    /// Returns a new less than comparison between two values.
    pub fn lt(lhs: impl Into<TypedValue>, rhs: impl Into<TypedValue>) -> Self {
        Self::new(lhs, ComparisonOperator::LessThan, rhs)
    }

    /// Returns a new less than or equals comparison between two values.
    pub fn le(lhs: impl Into<TypedValue>, rhs: impl Into<TypedValue>) -> Self {
        Self::new(lhs, ComparisonOperator::LessThanOrEquals, rhs)
    }

    /// Returns a new greater than comparison between two values.
    pub fn gt(lhs: impl Into<TypedValue>, rhs: impl Into<TypedValue>) -> Self {
        Self::new(lhs, ComparisonOperator::GreaterThan, rhs)
    }

    /// Returns a new greater than or equals comparison between two values.
    pub fn ge(lhs: impl Into<TypedValue>, rhs: impl Into<TypedValue>) -> Self {
        Self::new(lhs, ComparisonOperator::GreaterThanOrEquals, rhs)
    }

    /// Sets the given flag on this requirement.
    pub fn with_flag(mut self, flag: ComparisonFlag) -> Self {
        self.flag = Some(flag);
        self
    }

    /// Sets the hits on this requirement.
    pub fn with_hits(mut self, hits: u32) -> Self {
        self.hit_count.set_hits(hits);
        self
    }
}

impl FromStr for ComparisonRequirement {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_comparison_requirement
            .parse(s)
            .map_err(|s| ParseError::Requirement(s.to_string()))
    }
}

impl fmt::Display for ComparisonRequirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let flag = self.flag.map(|f| f.to_string()).unwrap_or_default();
        write!(
            f,
            "{}{}{}{}",
            flag, self.lhs, self.operation, self.hit_count
        )
    }
}

impl_comparison_flag_traits!(ComparisonRequirement, with_flag);

/// An operation in a comparison expression.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComparisonOperation {
    pub comparator: ComparisonOperator,
    pub rhs: TypedValue,
}

impl fmt::Display for ComparisonOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.comparator, self.rhs)
    }
}

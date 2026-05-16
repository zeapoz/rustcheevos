//! Type definition for comparison conditions.

pub(crate) use hits::HitCount;
use std::{fmt, str::FromStr};
use winnow::Parser;

pub(crate) mod hits;

use crate::{
    impl_condition_flag_traits,
    parsers::ParseError,
    parsers::parse_condition,
    types::{
        flag::ConditionFlag, memory::AccessMode, operator::ConditionOperator, value::TypedValue,
    },
};

use crate::types::memory::AccessModeModifier;

/// A comparison between two values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Condition {
    /// The flag of the comparison.
    flag: Option<ConditionFlag>,
    /// The left hand side of the comparison.
    lhs: TypedValue,
    /// The operation of the comparison, containing the operator and the right hand side expression.
    operation: ConditionOperation,
    /// The hit count of the comparison.
    hit_count: HitCount,
}

impl Condition {
    /// Returns a new comparison between two values.
    pub(crate) fn new(
        flag: Option<ConditionFlag>,
        lhs: impl Into<TypedValue>,
        operation: ConditionOperation,
        hit_count: HitCount,
    ) -> Self {
        Self {
            flag,
            lhs: lhs.into(),
            operation,
            hit_count,
        }
    }

    /// Returns a condition that always evaluates to true.
    ///
    /// This is a shorthand for a `1 = 1` comparison.
    #[must_use]
    pub fn always_true() -> Self {
        Self::eq(1, 1)
    }

    /// Returns a condition that always evaluates to false.
    ///
    /// This is a shorthand for a `0 = 1` comparison.
    #[must_use]
    pub fn always_false() -> Self {
        Self::eq(0, 1)
    }

    /// Returns a new equals comparison between two values.
    pub fn eq(lhs: impl Into<TypedValue>, rhs: impl Into<TypedValue>) -> Self {
        Self::new(
            None,
            lhs,
            ConditionOperation::new(ConditionOperator::Equals, rhs),
            HitCount::default(),
        )
    }

    /// Returns a new not equals comparison between two values.
    pub fn ne(lhs: impl Into<TypedValue>, rhs: impl Into<TypedValue>) -> Self {
        Self::new(
            None,
            lhs,
            ConditionOperation::new(ConditionOperator::NotEquals, rhs),
            HitCount::default(),
        )
    }

    /// Returns a new less than comparison between two values.
    pub fn lt(lhs: impl Into<TypedValue>, rhs: impl Into<TypedValue>) -> Self {
        Self::new(
            None,
            lhs,
            ConditionOperation::new(ConditionOperator::LessThan, rhs),
            HitCount::default(),
        )
    }

    /// Returns a new less than or equals comparison between two values.
    pub fn le(lhs: impl Into<TypedValue>, rhs: impl Into<TypedValue>) -> Self {
        Self::new(
            None,
            lhs,
            ConditionOperation::new(ConditionOperator::LessThanOrEquals, rhs),
            HitCount::default(),
        )
    }

    /// Returns a new greater than comparison between two values.
    pub fn gt(lhs: impl Into<TypedValue>, rhs: impl Into<TypedValue>) -> Self {
        Self::new(
            None,
            lhs,
            ConditionOperation::new(ConditionOperator::GreaterThan, rhs),
            HitCount::default(),
        )
    }

    /// Returns a new greater than or equals comparison between two values.
    pub fn ge(lhs: impl Into<TypedValue>, rhs: impl Into<TypedValue>) -> Self {
        Self::new(
            None,
            lhs,
            ConditionOperation::new(ConditionOperator::GreaterThanOrEquals, rhs),
            HitCount::default(),
        )
    }

    /// Returns the comparison flag if set.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let condition = Condition::eq(10, 10);
    /// assert_eq!(condition.flag(), None);
    /// ```
    #[must_use]
    pub fn flag(&self) -> Option<ConditionFlag> {
        self.flag
    }

    /// Returns the left hand side of the comparison.
    #[must_use]
    pub fn lhs(&self) -> &TypedValue {
        &self.lhs
    }

    /// Returns the right hand side of the comparison.
    #[must_use]
    pub fn rhs(&self) -> &TypedValue {
        &self.operation.rhs
    }

    /// Sets the given flag on this requirement.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let condition = Condition::eq(10, 10).add_hits();
    /// assert!(matches!(condition.flag(), Some(_)));
    /// ```
    #[must_use]
    pub fn with_flag(mut self, flag: ConditionFlag) -> Self {
        self.flag = Some(flag);
        self
    }

    /// Sets the hit count on this requirement.
    #[must_use]
    pub fn with_hits(mut self, hits: u32) -> Self {
        self.hit_count.set_hits(hits);
        self
    }
}

impl AccessModeModifier for Condition {
    fn with_access_mode(mut self, access_mode: AccessMode) -> Self {
        self.lhs = self.lhs.with_access_mode(access_mode);
        self.operation.rhs = self.operation.rhs.with_access_mode(access_mode);
        self
    }
}

impl FromStr for Condition {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_condition
            .parse(s)
            .map_err(|s| ParseError::Condition(s.to_string()))
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let flag = self.flag.map(|f| f.to_string()).unwrap_or_default();
        write!(
            f,
            "{}{}{}{}",
            flag, self.lhs, self.operation, self.hit_count
        )
    }
}

impl_condition_flag_traits!(Condition, with_flag);

/// An operation in a comparison expression.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct ConditionOperation {
    /// The operator of the comparison.
    operator: ConditionOperator,
    /// The right hand side of the comparison.
    rhs: TypedValue,
}

impl ConditionOperation {
    /// Creates a new comparison operation.
    pub(crate) fn new(operator: ConditionOperator, rhs: impl Into<TypedValue>) -> Self {
        Self {
            operator,
            rhs: rhs.into(),
        }
    }
}

impl fmt::Display for ConditionOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.operator, self.rhs)
    }
}

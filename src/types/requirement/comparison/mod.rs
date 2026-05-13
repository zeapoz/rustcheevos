//! Type definition for comparison requirements.

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

// TODO: Make fields private.
/// A comparison between two values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComparisonRequirement {
    /// The flag of the comparison.
    pub flag: Option<ComparisonFlag>,
    /// The left hand side of the comparison.
    pub lhs: TypedValue,
    /// The opertaion of the comparison, containing the operator and the right hand side expression.
    pub operation: ComparisonOperation,
    /// The hit count of the comparison.
    pub hit_count: HitCount,
}

impl ComparisonRequirement {
    /// Returns a new comparison between two values.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::operator::ComparisonOperator;
    /// # use rustcheevos::types::requirement::comparison::ComparisonRequirement;
    /// # use rustcheevos::types::value::TypedValue;
    /// let comparison = ComparisonRequirement::new(10, ComparisonOperator::Equals, 10);
    /// assert_eq!(comparison.lhs(), &TypedValue::from(10));
    /// assert_eq!(comparison.operator(), ComparisonOperator::Equals);
    /// assert_eq!(comparison.rhs(), &TypedValue::from(10));
    /// ```
    pub fn new(
        lhs: impl Into<TypedValue>,
        comparison: ComparisonOperator,
        rhs: impl Into<TypedValue>,
    ) -> Self {
        Self {
            flag: None,
            lhs: lhs.into(),
            operation: ComparisonOperation {
                operator: comparison,
                rhs: rhs.into(),
            },
            hit_count: HitCount::default(),
        }
    }

    /// Returns the comparison flag if set.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::operator::ComparisonOperator;
    /// # use rustcheevos::types::requirement::comparison::ComparisonRequirement;
    /// let comparison = ComparisonRequirement::new(10, ComparisonOperator::Equals, 10);
    /// assert_eq!(comparison.flag(), None);
    /// ```
    pub fn flag(&self) -> Option<ComparisonFlag> {
        self.flag
    }

    /// Returns the left hand side of the comparison.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::operator::ComparisonOperator;
    /// # use rustcheevos::types::requirement::comparison::ComparisonRequirement;
    /// # use rustcheevos::types::value::TypedValue;
    /// let comparison = ComparisonRequirement::new(10, ComparisonOperator::Equals, 10);
    /// assert_eq!(comparison.lhs(), &TypedValue::from(10));
    /// ```
    pub fn lhs(&self) -> &TypedValue {
        &self.lhs
    }

    /// Returns the comparison operator.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::operator::ComparisonOperator;
    /// # use rustcheevos::types::requirement::comparison::ComparisonRequirement;
    /// let comparison = ComparisonRequirement::new(10, ComparisonOperator::Equals, 10);
    /// assert_eq!(comparison.operator(), ComparisonOperator::Equals);
    /// ```
    pub fn operator(&self) -> ComparisonOperator {
        self.operation.operator
    }

    /// Returns the right hand side of the comparison.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::operator::ComparisonOperator;
    /// # use rustcheevos::types::requirement::comparison::ComparisonRequirement;
    /// # use rustcheevos::types::value::TypedValue;
    /// let comparison = ComparisonRequirement::new(10, ComparisonOperator::Equals, 10);
    /// assert_eq!(comparison.rhs(), &TypedValue::from(10));
    /// ```
    pub fn rhs(&self) -> &TypedValue {
        &self.operation.rhs
    }

    /// Returns the hit count.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::operator::ComparisonOperator;
    /// # use rustcheevos::types::requirement::comparison::ComparisonRequirement;
    /// let comparison = ComparisonRequirement::new(10, ComparisonOperator::Equals, 10);
    /// assert_eq!(*comparison.hit_count(), 0);
    /// ```
    pub fn hit_count(&self) -> HitCount {
        self.hit_count
    }

    /// Returns a new equals comparison between two values.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::requirement::comparison::ComparisonRequirement;
    /// let comparison = ComparisonRequirement::eq(10, 10);
    /// assert_eq!(comparison.operator().to_string(), "=");
    /// ```
    pub fn eq(lhs: impl Into<TypedValue>, rhs: impl Into<TypedValue>) -> Self {
        Self::new(lhs, ComparisonOperator::Equals, rhs)
    }

    /// Returns a new not equals comparison between two values.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::requirement::comparison::ComparisonRequirement;
    /// let comparison = ComparisonRequirement::ne(10, 10);
    /// assert_eq!(comparison.operator().to_string(), "!=");
    /// ```
    pub fn ne(lhs: impl Into<TypedValue>, rhs: impl Into<TypedValue>) -> Self {
        Self::new(lhs, ComparisonOperator::NotEquals, rhs)
    }

    /// Returns a new less than comparison between two values.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::requirement::comparison::ComparisonRequirement;
    /// let comparison = ComparisonRequirement::lt(10, 10);
    /// assert_eq!(comparison.operator().to_string(), "<");
    /// ```
    pub fn lt(lhs: impl Into<TypedValue>, rhs: impl Into<TypedValue>) -> Self {
        Self::new(lhs, ComparisonOperator::LessThan, rhs)
    }

    /// Returns a new less than or equals comparison between two values.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::requirement::comparison::ComparisonRequirement;
    /// let comparison = ComparisonRequirement::le(10, 10);
    /// assert_eq!(comparison.operator().to_string(), "<=");
    /// ```
    pub fn le(lhs: impl Into<TypedValue>, rhs: impl Into<TypedValue>) -> Self {
        Self::new(lhs, ComparisonOperator::LessThanOrEquals, rhs)
    }

    /// Returns a new greater than comparison between two values.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::requirement::comparison::ComparisonRequirement;
    /// let comparison = ComparisonRequirement::gt(10, 10);
    /// assert_eq!(comparison.operator().to_string(), ">");
    /// ```
    pub fn gt(lhs: impl Into<TypedValue>, rhs: impl Into<TypedValue>) -> Self {
        Self::new(lhs, ComparisonOperator::GreaterThan, rhs)
    }

    /// Returns a new greater than or equals comparison between two values.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::requirement::comparison::ComparisonRequirement;
    /// let comparison = ComparisonRequirement::ge(10, 10);
    /// assert_eq!(comparison.operator().to_string(), ">=");
    /// ```
    pub fn ge(lhs: impl Into<TypedValue>, rhs: impl Into<TypedValue>) -> Self {
        Self::new(lhs, ComparisonOperator::GreaterThanOrEquals, rhs)
    }

    /// Sets the given flag on this requirement.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::operator::ComparisonOperator;
    /// # use rustcheevos::types::flag::ComparisonFlag;
    /// # use rustcheevos::types::requirement::comparison::ComparisonRequirement;
    /// # use rustcheevos::types::value::TypedValue;
    /// let comparison = ComparisonRequirement::new(10, ComparisonOperator::Equals, 10);
    /// assert_eq!(comparison.flag(), None);
    /// let comparison = comparison.with_flag(ComparisonFlag::AddHits);
    /// assert_eq!(comparison.flag(), Some(ComparisonFlag::AddHits));
    /// ```
    pub fn with_flag(mut self, flag: ComparisonFlag) -> Self {
        self.flag = Some(flag);
        self
    }

    /// Sets the hit count on this requirement.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::operator::ComparisonOperator;
    /// # use rustcheevos::types::requirement::comparison::ComparisonRequirement;
    /// # use rustcheevos::types::value::TypedValue;
    /// let comparison = ComparisonRequirement::new(10, ComparisonOperator::Equals, 10);
    /// assert_eq!(*comparison.hit_count(), 0);
    /// let comparison = comparison.with_hits(10);
    /// assert_eq!(*comparison.hit_count(), 10);
    /// ```
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
    /// The operator of the comparison.
    pub operator: ComparisonOperator,
    /// The right hand side of the comparison.
    pub rhs: TypedValue,
}

impl fmt::Display for ComparisonOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.operator, self.rhs)
    }
}

use std::{fmt, str::FromStr};

use crate::{
    ParseError, impl_arithmetic_flag_traits, impl_comparison_flag_traits,
    types::flag::{ArithmeticFlag, ComparisonFlag},
};

use super::Requirement;

/// A group of requirements that must all be true.
#[derive(Debug, Clone, PartialEq)]
pub struct RequirementGroup(Vec<Requirement>);

impl RequirementGroup {
    /// Creates a new condition group
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Pushes a new requirement to the group.
    ///
    /// # Arguments
    ///
    /// * `requirement` - The requirement to push.
    pub fn push(&mut self, requirement: Requirement) {
        self.0.push(requirement);
    }

    /// Extends the group with the given requirements.
    ///
    /// # Arguments
    ///
    /// * `iter` - The requirements to extend the group with.
    pub fn extend(&mut self, item: impl Into<RequirementGroup>) {
        self.0.extend_from_slice(&item.into().into_inner());
    }

    /// Returns an iterator over the conditions in this group.
    ///
    /// # Returns
    ///
    /// An iterator over references to the conditions.
    pub fn iter(&self) -> impl Iterator<Item = &Requirement> {
        self.0.iter()
    }

    /// Consumes this group and returns the inner conditions.
    ///
    /// # Returns
    ///
    /// The inner conditions.
    pub fn into_inner(self) -> Vec<Requirement> {
        self.0
    }

    /// Sets the comparison flag for all the inner comparison requirements.
    pub fn with_comparison_flag(self, flag: ComparisonFlag) -> Self {
        self.0
            .into_iter()
            .map(|r| {
                if let Requirement::Comparison(comparison) = r {
                    Requirement::Comparison(comparison.with_flag(flag))
                } else {
                    r
                }
            })
            .collect()
    }

    /// Sets the arithemetic flag for all the inner arithmetic requirements.
    pub fn with_arithmetic_flag(self, flag: ArithmeticFlag) -> Self {
        self.0
            .into_iter()
            .map(|r| {
                if let Requirement::Arithmetic(arithmetic) = r {
                    Requirement::Arithmetic(arithmetic.with_flag(flag))
                } else {
                    r
                }
            })
            .collect()
    }
}

impl<T: Into<Requirement>> From<T> for RequirementGroup {
    fn from(value: T) -> Self {
        RequirementGroup(vec![value.into()])
    }
}

impl<const N: usize, T: Into<Requirement>> From<[T; N]> for RequirementGroup {
    fn from(arr: [T; N]) -> Self {
        let arr = arr.into_iter().map(T::into).collect::<Vec<_>>();
        RequirementGroup(arr.into())
    }
}

impl From<Vec<Requirement>> for RequirementGroup {
    fn from(value: Vec<Requirement>) -> Self {
        RequirementGroup(value)
    }
}

impl FromIterator<Requirement> for RequirementGroup {
    fn from_iter<T: IntoIterator<Item = Requirement>>(iter: T) -> Self {
        RequirementGroup(iter.into_iter().collect())
    }
}

impl FromStr for RequirementGroup {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let requirement: Vec<_> = s
            .split('_')
            .filter(|s| !s.is_empty())
            .map(Requirement::from_str)
            .collect::<Result<_, _>>()?;

        Ok(Self(requirement))
    }
}

impl fmt::Display for RequirementGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|r| r.to_string())
                .collect::<Vec<_>>()
                .join("_")
        )
    }
}

impl_comparison_flag_traits!(RequirementGroup, with_comparison_flag);
impl_arithmetic_flag_traits!(RequirementGroup, with_arithmetic_flag);

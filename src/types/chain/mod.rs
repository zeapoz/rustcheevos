use std::{fmt, str::FromStr};

use crate::{
    impl_arithmetic_flag_traits, impl_comparison_flag_traits,
    parsers::ParseError,
    prelude::Requirement,
    types::flag::{ArithmeticFlag, ComparisonFlag},
};

pub mod pending;

/// A holding struct for many groups of requirements.
#[derive(Debug, Clone, PartialEq)]
pub struct ChainGroup {
    core: Chain,
    alt_groups: Vec<Chain>,
}

impl ChainGroup {
    /// Creates a new chain with the given core chain.
    ///
    /// # Arguments
    ///
    /// * `core` - The core chain.
    pub fn new(core: impl Into<Chain>) -> Self {
        Self {
            core: core.into(),
            alt_groups: Vec::new(),
        }
    }

    /// Adds an alternative group of requirements.
    ///
    /// # Arguments
    ///
    /// * `alt_group` - The alternative group of requirements.
    pub fn push_alt_group(&mut self, group: Chain) {
        self.alt_groups.push(group);
    }

    /// Adds multiple alternative groups of requirements.
    ///
    /// # Arguments
    ///
    /// * `alt_groups` - The alternative groups of requirements.
    pub fn with_alt_groups(
        mut self,
        alt_groups: impl IntoIterator<Item = impl Into<Chain>>,
    ) -> Self {
        self.alt_groups = alt_groups.into_iter().map(Into::into).collect();
        self
    }
}

impl<T: Into<Chain>> From<T> for ChainGroup {
    fn from(value: T) -> Self {
        ChainGroup::new(value.into())
    }
}

impl fmt::Display for ChainGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let alts: String = self.alt_groups.iter().map(|g| format!("S{g}")).collect();
        write!(f, "{}{}", self.core, alts)
    }
}

/// A chain of requirements that must all be true.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Chain(Vec<Requirement>);

impl Chain {
    /// Creates a new chain
    pub fn new() -> Self {
        Self::default()
    }

    /// Pushes a new requirement to the group.
    ///
    /// # Arguments
    ///
    /// * `requirement` - The requirement to push.
    pub fn push(&mut self, requirement: impl Into<Requirement>) {
        self.0.push(requirement.into());
    }

    /// Extends the group with the given requirements.
    ///
    /// # Arguments
    ///
    /// * `item` - The requirements to extend the group with.
    pub fn extend(&mut self, item: impl Into<Chain>) {
        self.0.extend_from_slice(&item.into().into_inner());
    }

    /// Returns an iterator over the requirements in this group.
    ///
    /// # Returns
    ///
    /// An iterator over references to the requirements.
    pub fn iter(&self) -> impl Iterator<Item = &Requirement> {
        self.0.iter()
    }

    /// Consumes this group and returns the inner requirements.
    ///
    /// # Returns
    ///
    /// The inner requirements.
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

impl<T: Into<Requirement>> From<T> for Chain {
    fn from(value: T) -> Self {
        Chain(vec![value.into()])
    }
}

impl<const N: usize, T: Into<Requirement>> From<[T; N]> for Chain {
    fn from(arr: [T; N]) -> Self {
        let arr = arr.into_iter().map(T::into).collect::<Vec<_>>();
        Chain(arr)
    }
}

impl From<Vec<Requirement>> for Chain {
    fn from(value: Vec<Requirement>) -> Self {
        Chain(value)
    }
}

impl<T: Into<Chain>> FromIterator<T> for Chain {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let chains: Vec<_> = iter.into_iter().map(T::into).collect();
        Chain(chains.into_iter().flat_map(Chain::into_inner).collect())
    }
}

impl FromStr for Chain {
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

impl fmt::Display for Chain {
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

impl_comparison_flag_traits!(Chain, with_comparison_flag);
impl_arithmetic_flag_traits!(Chain, with_arithmetic_flag);

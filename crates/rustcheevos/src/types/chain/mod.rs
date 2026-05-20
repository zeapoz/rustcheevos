//! Type definitions for requirement chains.

use std::{fmt, str::FromStr};

use crate::{
    impl_arithmetic_flag_traits, impl_condition_flag_traits,
    parsers::ParseError,
    types::{
        flag::{ArithmeticFlag, ConditionFlag},
        memory::AccessModeModifier,
    },
    types::{memory::AccessMode, requirement::Requirement},
};

pub(crate) mod pending;

pub use pending::{Chainable, PendingChain};

/// A holding struct for many groups of requirements.
///
/// This type is used to group requirements together for use in an
/// [Achievement][`crate::types::achievement::Achievement`].
///
/// # Examples
///
/// ```
/// use rustcheevos::prelude::*;
/// use rustcheevos::types::chain::ChainGroup;
/// use rustcheevos::{bits8, chain, delta};
///
/// let core_condition = chain!(
///     delta!(bits8!(0x1234)).lt(10),
///     bits8!(0x1234).ge(10),
/// );
///
/// let alt_condition_a = chain!(
///     delta!(bits8!(0x1234)).lt(10),
///     bits8!(0x1234).ge(10),
/// );
///
/// let alt_condition_b = chain!(
///     delta!(bits8!(0x1234)).lt(10),
///     bits8!(0x1234).ge(10),
/// );
///
/// let mut chain_group = ChainGroup::new(core_condition);
/// chain_group.push_alt_group(alt_condition_a);
/// chain_group.push_alt_group(alt_condition_b);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ChainGroup {
    /// The core group.
    core: Chain,
    /// The alternative groups.
    alt_groups: Vec<Chain>,
}

impl ChainGroup {
    /// Creates a new group with the given core chain.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::ChainGroup;
    /// use rustcheevos::{bits8, chain, delta};
    ///
    /// let core_condition = chain!(
    ///     delta!(bits8!(0x1234)).lt(10),
    ///     bits8!(0x1234).ge(10),
    /// );
    ///
    /// let chain_group = ChainGroup::new(core_condition);
    /// ```
    pub fn new(core: impl Into<Chain>) -> Self {
        Self {
            core: core.into(),
            alt_groups: Vec::new(),
        }
    }

    /// Adds an alternative group of requirements.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::chain::{Chain, ChainGroup};
    /// # let core_condition = Chain::default();
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::{bits8, chain, delta};
    ///
    /// let alt_group = chain!(
    ///     delta!(bits8!(0x1234)).lt(10),
    ///     bits8!(0x1234).ge(10),
    /// );
    ///
    /// let mut chain_group = ChainGroup::new(core_condition);
    /// chain_group.push_alt_group(alt_group);
    /// ```
    pub fn push_alt_group(&mut self, group: impl Into<Chain>) {
        self.alt_groups.push(group.into());
    }

    /// Adds multiple alternative groups of requirements to this chain group.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::chain::{Chain, ChainGroup};
    /// # let core_condition = Chain::default();
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::{bits8, chain, delta};
    ///
    /// let alt_group_a = chain!(
    ///     delta!(bits8!(0x1234)).lt(10),
    ///     bits8!(0x1234).ge(10),
    /// );
    ///
    /// let alt_group_b = chain!(
    ///     delta!(bits8!(0x1234)).lt(10),
    ///     bits8!(0x1234).ge(10),
    /// );
    ///
    /// let mut chain_group = ChainGroup::new(core_condition);
    /// chain_group.set_alt_groups(vec![alt_group_a, alt_group_b]);
    /// ```
    pub fn set_alt_groups(&mut self, alt_groups: impl IntoIterator<Item = impl Into<Chain>>) {
        self.alt_groups = alt_groups.into_iter().map(Into::into).collect();
    }
}

impl<T: Into<Chain>> From<T> for ChainGroup {
    fn from(value: T) -> Self {
        ChainGroup::new(value.into())
    }
}

impl fmt::Display for ChainGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.core)?;
        for g in &self.alt_groups {
            write!(f, "S{g}")?;
        }
        Ok(())
    }
}

/// A chain of requirements that must all be true.
///
/// This type is used to group requirements together for use in an
/// [Achievement][`crate::types::achievement::Achievement`].
///
/// While this type can be used directly, it is recommend to use the
/// [`chain!`][`crate::chain!`] macro instead for better ergonomics.
///
/// # Examples
///
/// ```
/// use rustcheevos::prelude::*;
/// use rustcheevos::types::chain::Chain;
/// use rustcheevos::{bits8, chain, delta};
///
/// let chain_a = chain!(
///     delta!(bits8!(0x1234)).lt(10),
///     bits8!(0x1234).ge(10),
/// );
///
/// let mut chain_b = Chain::new();
/// chain_b.push(delta!(bits8!(0x1234)).lt(10));
/// chain_b.push(bits8!(0x1234).ge(10));
///
/// assert_eq!(chain_a, chain_b);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Chain(Vec<Requirement>);

impl Chain {
    /// Creates a new chain.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::Chain;
    /// use rustcheevos::{bits8, chain, delta};
    ///
    /// let mut chain = Chain::new();
    ///
    /// let requirement = delta!(bits8!(0x1234)).lt(10);
    /// chain.push(requirement);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Pushes a new requirement to the chain.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::Chain;
    /// use rustcheevos::{bits8, chain, delta};
    ///
    /// let requirement = delta!(bits8!(0x1234)).lt(10);
    ///
    /// let mut chain = Chain::new();
    /// chain.push(requirement);
    /// ```
    pub fn push(&mut self, requirement: impl Into<Requirement>) {
        self.0.push(requirement.into());
    }

    /// Extends the chain with the given requirements.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::Chain;
    /// use rustcheevos::{bits8, chain, delta};
    ///
    /// let mut chain_a = chain!(
    ///     delta!(bits8!(0x1234)).lt(10),
    ///     bits8!(0x1234).ge(10),
    /// );
    ///
    ///
    /// let chain_b = chain!(
    ///     delta!(bits8!(0x1234)).lt(10),
    ///     bits8!(0x1234).ge(10),
    /// );
    ///
    /// chain_a.extend(chain_b);
    /// ```
    ///
    pub fn extend(&mut self, item: impl Into<Chain>) {
        self.0.extend_from_slice(&item.into().into_inner());
    }

    /// Returns an iterator over the requirements in this chain.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::Chain;
    /// use rustcheevos::{bits8, chain, delta};
    ///
    /// let chain = chain!(
    ///     delta!(bits8!(0x1234)).lt(10),
    ///     bits8!(0x1234).ge(10),
    /// );
    ///
    /// chain.iter().for_each(|requirement| {
    ///     println!("{requirement}");
    /// });
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &Requirement> {
        self.0.iter()
    }

    /// Consumes this chain and returns the inner requirements.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::Chain;
    /// use rustcheevos::{bits8, chain, delta};
    ///
    /// let requirement = delta!(bits8!(0x1234)).lt(10);
    ///
    /// let chain = chain!(requirement.clone());
    /// assert_eq!(chain.into_inner(), vec![requirement.into()]);
    /// ```
    #[must_use]
    pub fn into_inner(self) -> Vec<Requirement> {
        self.0
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

impl<T: Into<Requirement>> From<Vec<T>> for Chain {
    fn from(value: Vec<T>) -> Self {
        let value = value.into_iter().map(T::into).collect::<Vec<_>>();
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
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("_")
        )
    }
}

impl AccessModeModifier for Chain {
    fn with_access_mode(mut self, access_mode: AccessMode) -> Self {
        for req in &mut self.0 {
            *req = req.with_access_mode(access_mode);
        }
        self
    }
}

impl Chain {
    /// Sets the given comparison flag on all [`Condition`](crate::types::requirement::Condition) requirements in this chain.
    ///
    /// [`Arithmetic`](crate::types::requirement::Arithmetic) requirements are returned unchanged.
    #[must_use]
    pub fn with_condition_flag(self, flag: ConditionFlag) -> Self {
        Self(
            self.0
                .into_iter()
                .map(|req| req.with_condition_flag(flag))
                .collect(),
        )
    }

    /// Sets the given arithmetic flag on all [`Arithmetic`](crate::types::requirement::Arithmetic) requirements in this chain.
    ///
    /// [`Condition`](crate::types::requirement::Condition) requirements are returned unchanged.
    #[must_use]
    pub fn with_arithmetic_flag(self, flag: ArithmeticFlag) -> Self {
        Self(
            self.0
                .into_iter()
                .map(|req| req.with_arithmetic_flag(flag))
                .collect(),
        )
    }
}

impl_condition_flag_traits!(Chain, with_condition_flag);
impl_arithmetic_flag_traits!(Chain, with_arithmetic_flag);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_chain_single_requirement() {
        let original: Chain = "0xH1234=50".parse().unwrap();
        let serialized = original.to_string();
        let parsed: Chain = serialized.parse().unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn roundtrip_chain_multiple_requirements() {
        let original: Chain = "0xH1234=50_d0xH1234>=10".parse().unwrap();
        let serialized = original.to_string();
        let parsed: Chain = serialized.parse().unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn roundtrip_chain_with_arithmetic() {
        let original: Chain = "A:0xH1234+10_0xH5678=0".parse().unwrap();
        let serialized = original.to_string();
        let parsed: Chain = serialized.parse().unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn roundtrip_chain_with_hit_count() {
        let original: Chain = "0xH1234=1.100._0xH5678>=5".parse().unwrap();
        let serialized = original.to_string();
        let parsed: Chain = serialized.parse().unwrap();
        assert_eq!(original, parsed);
    }
}

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

pub use pending::{Chain, Chainable};

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
/// use rustcheevos::types::{chain::{Chain, ResolvedChain}, requirement::Condition};
/// use rustcheevos::{bits8, chain, delta};
///
/// // A reusable helper keeps its head pending, so it can still be modified
/// // at the call site before being resolved into a [`ResolvedChain`].
/// fn digging() -> Chain<Condition> {
///     chain!(
///         and_next!(bits8!(0x10).eq(1)),
///         bits8!(0x11).eq(1),
///     )
/// }
///
/// let resolved: ResolvedChain = digging().with_hits(5).into();
/// ```
///
/// ```
/// use rustcheevos::prelude::*;
/// use rustcheevos::types::chain::ResolvedChain;
/// use rustcheevos::{bits8, chain, delta};
///
/// let chain_a: ResolvedChain = chain!(
///     delta!(bits8!(0x1234)).lt(10),
///     bits8!(0x1234).ge(10),
/// )
/// .into();
///
/// let mut chain_b = ResolvedChain::new();
/// chain_b.push(delta!(bits8!(0x1234)).lt(10));
/// chain_b.push(bits8!(0x1234).ge(10));
///
/// assert_eq!(chain_a, chain_b);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ResolvedChain(Vec<Requirement>);

impl ResolvedChain {
    /// Creates a new chain.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::ResolvedChain;
    /// use rustcheevos::{bits8, chain, delta};
    ///
    /// let mut chain = ResolvedChain::new();
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
    /// use rustcheevos::types::chain::ResolvedChain;
    /// use rustcheevos::{bits8, chain, delta};
    ///
    /// let requirement = delta!(bits8!(0x1234)).lt(10);
    ///
    /// let mut chain = ResolvedChain::new();
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
    /// use rustcheevos::types::chain::ResolvedChain;
    /// use rustcheevos::{bits8, chain, delta};
    ///
    /// let mut chain_a: ResolvedChain = chain!(
    ///     delta!(bits8!(0x1234)).lt(10),
    ///     bits8!(0x1234).ge(10),
    /// )
    /// .into();
    ///
    ///
    /// let chain_b: ResolvedChain = chain!(
    ///     delta!(bits8!(0x1234)).lt(10),
    ///     bits8!(0x1234).ge(10),
    /// )
    /// .into();
    ///
    /// chain_a.extend(chain_b);
    /// ```
    ///
    pub fn extend(&mut self, item: impl Into<ResolvedChain>) {
        self.0.extend_from_slice(&item.into().into_inner());
    }

    /// Returns an iterator over the requirements in this chain.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::ResolvedChain;
    /// use rustcheevos::{bits8, chain, delta};
    ///
    /// let chain: ResolvedChain = chain!(
    ///     delta!(bits8!(0x1234)).lt(10),
    ///     bits8!(0x1234).ge(10),
    /// )
    /// .into();
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
    /// use rustcheevos::types::chain::ResolvedChain;
    /// use rustcheevos::{bits8, chain, delta};
    ///
    /// let requirement = delta!(bits8!(0x1234)).lt(10);
    ///
    /// let chain: ResolvedChain = chain!(requirement.clone()).into();
    /// assert_eq!(chain.into_inner(), vec![requirement.into()]);
    /// ```
    #[must_use]
    pub fn into_inner(self) -> Vec<Requirement> {
        self.0
    }

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

impl<T: Into<Requirement>> From<T> for ResolvedChain {
    fn from(value: T) -> Self {
        ResolvedChain(vec![value.into()])
    }
}

impl<const N: usize, T: Into<Requirement>> From<[T; N]> for ResolvedChain {
    fn from(arr: [T; N]) -> Self {
        let arr = arr.into_iter().map(T::into).collect::<Vec<_>>();
        ResolvedChain(arr)
    }
}

impl<T: Into<Requirement>> From<Vec<T>> for ResolvedChain {
    fn from(value: Vec<T>) -> Self {
        let value = value.into_iter().map(T::into).collect::<Vec<_>>();
        ResolvedChain(value)
    }
}

impl<T: Into<ResolvedChain>> FromIterator<T> for ResolvedChain {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let chains: Vec<_> = iter.into_iter().map(T::into).collect();
        ResolvedChain(
            chains
                .into_iter()
                .flat_map(ResolvedChain::into_inner)
                .collect(),
        )
    }
}

impl AccessModeModifier for ResolvedChain {
    fn with_access_mode(mut self, access_mode: AccessMode) -> Self {
        for req in &mut self.0 {
            *req = req.with_access_mode(access_mode);
        }
        self
    }
}

impl FromStr for ResolvedChain {
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

impl fmt::Display for ResolvedChain {
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

impl_condition_flag_traits!(ResolvedChain, with_condition_flag);
impl_arithmetic_flag_traits!(ResolvedChain, with_arithmetic_flag);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_chain_single_requirement() {
        let original: ResolvedChain = "0xH1234=50".parse().unwrap();
        let serialized = original.to_string();
        let parsed: ResolvedChain = serialized.parse().unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn roundtrip_chain_multiple_requirements() {
        let original: ResolvedChain = "0xH1234=50_d0xH1234>=10".parse().unwrap();
        let serialized = original.to_string();
        let parsed: ResolvedChain = serialized.parse().unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn roundtrip_chain_with_arithmetic() {
        let original: ResolvedChain = "A:0xH1234+10_0xH5678=0".parse().unwrap();
        let serialized = original.to_string();
        let parsed: ResolvedChain = serialized.parse().unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn roundtrip_chain_with_hit_count() {
        let original: ResolvedChain = "0xH1234=1.100._0xH5678>=5".parse().unwrap();
        let serialized = original.to_string();
        let parsed: ResolvedChain = serialized.parse().unwrap();
        assert_eq!(original, parsed);
    }
}

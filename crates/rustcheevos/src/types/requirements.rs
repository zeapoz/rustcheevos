//! Type definitions for requirement sets.

use std::fmt;

use crate::types::chain::ResolvedChain;

/// A set of requirement chains defining when a condition is satisfied.
///
/// Contains a core chain that must always be satisfied, plus zero or more
/// alternative chains. If any alternative chains are present, at least one
/// of them must also be satisfied.
///
/// # Examples
///
/// ```
/// use rustcheevos::prelude::*;
/// use rustcheevos::types::requirements::Requirements;
/// use rustcheevos::{bits8, chain, delta};
///
/// let core = chain!(
///     delta!(bits8!(0x1234)).lt(10),
///     bits8!(0x1234).ge(10),
/// );
///
/// let alt_a = chain!(
///     delta!(bits8!(0x1234)).lt(10),
///     bits8!(0x1234).ge(10),
/// );
///
/// let alt_b = chain!(
///     delta!(bits8!(0x1234)).lt(10),
///     bits8!(0x1234).ge(10),
/// );
///
/// let requirements = Requirements::new(core).with_alt(alt_a).with_alt(alt_b);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Requirements {
    /// The core group.
    core: ResolvedChain,
    /// The alternative groups.
    alt_groups: Vec<ResolvedChain>,
}

impl Requirements {
    /// Creates a new set with the given core chain.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::requirements::Requirements;
    /// use rustcheevos::{bits8, chain, delta};
    ///
    /// let core = chain!(
    ///     delta!(bits8!(0x1234)).lt(10),
    ///     bits8!(0x1234).ge(10),
    /// );
    ///
    /// let requirements = Requirements::new(core);
    /// ```
    pub fn new(core: impl Into<ResolvedChain>) -> Self {
        Self {
            core: core.into(),
            alt_groups: Vec::new(),
        }
    }

    /// Replaces the core chain while preserving any alternative groups.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::chain::ResolvedChain;
    /// # use rustcheevos::types::requirements::Requirements;
    /// # let core = ResolvedChain::default();
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::{bits8, chain, delta};
    ///
    /// let alt = chain!(bits8!(0x1234).eq(1));
    /// let requirements = Requirements::new(core).with_alt(alt).with_core(chain!(bits8!(0x1234).eq(2)));
    /// ```
    pub fn with_core(self, core: impl Into<ResolvedChain>) -> Self {
        Self {
            core: core.into(),
            alt_groups: self.alt_groups,
        }
    }

    /// Adds an alternative chain group.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::chain::ResolvedChain;
    /// # use rustcheevos::types::requirements::Requirements;
    /// # let core = ResolvedChain::default();
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::{bits8, chain, delta};
    ///
    /// let alt = chain!(
    ///     delta!(bits8!(0x1234)).lt(10),
    ///     bits8!(0x1234).ge(10),
    /// );
    ///
    /// let requirements = Requirements::new(core).with_alt(alt);
    /// ```
    pub fn with_alt(mut self, group: impl Into<ResolvedChain>) -> Self {
        self.alt_groups.push(group.into());
        self
    }

    /// Adds multiple alternative chain groups.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::chain::ResolvedChain;
    /// # use rustcheevos::types::requirements::Requirements;
    /// # let core = ResolvedChain::default();
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::{bits8, chain, delta};
    ///
    /// let alt_a = chain!(
    ///     delta!(bits8!(0x1234)).lt(10),
    ///     bits8!(0x1234).ge(10),
    /// );
    ///
    /// let alt_b = chain!(
    ///     delta!(bits8!(0x1234)).lt(10),
    ///     bits8!(0x1234).ge(10),
    /// );
    ///
    /// let requirements = Requirements::new(core).with_alts([alt_a, alt_b]);
    /// ```
    pub fn with_alts(mut self, groups: impl IntoIterator<Item = impl Into<ResolvedChain>>) -> Self {
        self.alt_groups.extend(groups.into_iter().map(Into::into));
        self
    }

    /// Returns the core chain.
    #[must_use]
    pub fn core(&self) -> &ResolvedChain {
        &self.core
    }

    /// Returns the alternative chain groups.
    #[must_use]
    pub fn alt_groups(&self) -> &[ResolvedChain] {
        &self.alt_groups
    }
}

impl<T: Into<ResolvedChain>> From<T> for Requirements {
    fn from(value: T) -> Self {
        Requirements::new(value.into())
    }
}

impl fmt::Display for Requirements {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.core)?;
        for g in &self.alt_groups {
            write!(f, "S{g}")?;
        }
        Ok(())
    }
}

//! Type definitions for requirement sets.

use std::fmt;

use crate::types::chain::Chain;

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
/// let mut requirements = Requirements::new(core);
/// requirements.add_alt_group(alt_a);
/// requirements.add_alt_group(alt_b);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Requirements {
    /// The core group.
    core: Chain,
    /// The alternative groups.
    alt_groups: Vec<Chain>,
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
    pub fn new(core: impl Into<Chain>) -> Self {
        Self {
            core: core.into(),
            alt_groups: Vec::new(),
        }
    }

    /// Adds an alternative chain group.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::chain::Chain;
    /// # use rustcheevos::types::requirements::Requirements;
    /// # let core = Chain::default();
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::{bits8, chain, delta};
    ///
    /// let alt = chain!(
    ///     delta!(bits8!(0x1234)).lt(10),
    ///     bits8!(0x1234).ge(10),
    /// );
    ///
    /// let mut requirements = Requirements::new(core);
    /// requirements.add_alt_group(alt);
    /// ```
    pub fn add_alt_group(&mut self, group: impl Into<Chain>) -> &mut Self {
        self.alt_groups.push(group.into());
        self
    }

    /// Adds multiple alternative chain groups.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::chain::Chain;
    /// # use rustcheevos::types::requirements::Requirements;
    /// # let core = Chain::default();
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
    /// let mut requirements = Requirements::new(core);
    /// requirements.add_alt_groups([alt_a, alt_b]);
    /// ```
    pub fn add_alt_groups(
        &mut self,
        groups: impl IntoIterator<Item = impl Into<Chain>>,
    ) -> &mut Self {
        self.alt_groups.extend(groups.into_iter().map(Into::into));
        self
    }

    /// Returns the core chain.
    #[must_use]
    pub fn core(&self) -> &Chain {
        &self.core
    }

    /// Returns the alternative chain groups.
    #[must_use]
    pub fn alt_groups(&self) -> &[Chain] {
        &self.alt_groups
    }
}

impl<T: Into<Chain>> From<T> for Requirements {
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

//! Type definition for pending chains.

use crate::{
    impl_arithmetic_flag_traits, impl_condition_flag_traits,
    types::{
        chain::ResolvedChain,
        flag::{ArithmeticFlag, ConditionFlag, Measured},
        memory::{AccessMode, AccessModeModifier, MemoryRef},
        requirement::{Requirement, arithmetic::Arithmetic, condition::Condition},
        value::{TypedValue, TypedValueOps},
    },
};

/// A trait for types that can be chained in a [`ResolvedChain`].
pub trait Chainable {
    /// The output type.
    type Output;

    /// Chains the type with the given chain.
    fn chain(self, chain: ResolvedChain) -> Self::Output;
}

/// A pending chain of requirements.
///
/// This type is a specialized version of [`ResolvedChain`] that is used to build and compose chains
/// of requirements where the head of the chain can be still be modified.
///
/// ```
/// # use rustcheevos::bits8;
/// # const BASE_ADDR: usize = 0x0;
/// # const PROFILE_STRIDE: u32 = 0x0;
/// # #[derive(Clone, Copy)]
/// # enum Addr { Zero = 0 }
/// # fn current_profile() -> MemoryRef { bits8!(0x0) }
/// use rustcheevos::prelude::*;
/// use rustcheevos::types::{chain::{Chain, ResolvedChain}, memory::MemoryRef};
/// use rustcheevos::{add_address, bits32, chain};
/// # impl Addr {
///
/// // Define a pending chain, with the head being a memory reference.
/// pub fn level(&self) -> Chain<MemoryRef> {
///     let offset = BASE_ADDR + *self as usize * 4;
///     chain!(
///         add_address!(current_profile().mul(PROFILE_STRIDE)),
///         bits32!(offset)
///     )
/// }
///
/// // The head of the chain can be modified to construct a new resolved chain.
/// pub fn is_level(&self, level: u32) -> ResolvedChain {
///     self.level().eq(level).into()
/// }
/// # }
#[derive(Debug, Clone, PartialEq)]
pub struct Chain<T> {
    /// The head of the chain.
    head: T,
    /// The pending chain.
    pending: ResolvedChain,
}

impl<T> Chain<T> {
    /// Creates a new pending chain.
    ///
    /// # Exampless
    ///
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, ResolvedChain};
    /// use rustcheevos::{add_address, bits32, chain};
    ///
    /// let chain = chain!(
    ///     add_address!(bits32!(0x1234)),
    ///     bits32!(0x5432).eq(0)
    /// );
    ///
    /// Chain::new(0, chain);
    pub fn new(head: T, pending: impl Into<ResolvedChain>) -> Self {
        Self {
            head,
            pending: pending.into(),
        }
    }

    /// Returns the head of the chain.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, ResolvedChain};
    /// use rustcheevos::{add_address, bits32, chain};
    ///
    /// let chain = chain!(
    ///     add_address!(bits32!(0x1234)),
    ///     bits32!(0x5432).eq(0)
    /// );
    ///
    /// let pending_chain = Chain::new(0, chain);
    /// assert_eq!(*pending_chain.head(), 0);
    /// ```
    pub fn head(&self) -> &T {
        &self.head
    }

    /// Returns the pending chain.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, ResolvedChain};
    /// use rustcheevos::{add_address, bits32, chain};
    ///
    /// let chain = chain!(bits32!(0x5432).eq(0));
    ///
    /// let pending_chain = Chain::new(0, chain);
    /// assert_eq!(pending_chain.pending(), &chain!(bits32!(0x5432).eq(0)).into());
    /// ```
    pub fn pending(&self) -> &ResolvedChain {
        &self.pending
    }
}

impl<T: Into<Requirement>> Chain<T> {
    /// Resolves the pending chain into a [`ResolvedChain`] by appending the head.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, ResolvedChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let pending = chain!(bits8!(0x1234).eq(0));
    /// let resolved: ResolvedChain = pending.resolve();
    /// assert_eq!(resolved, ResolvedChain::from(bits8!(0x1234).eq(0)));
    /// ```
    #[must_use]
    pub fn resolve(self) -> ResolvedChain {
        let mut chain = self.pending;
        chain.extend(self.head);
        chain
    }
}

impl Chain<ResolvedChain> {
    /// Resolves a nested pending chain into a [`ResolvedChain`] by appending the head.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, ResolvedChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let base: ResolvedChain = chain!(bits8!(0x1234).eq(0)).into();
    /// let pending = Chainable::chain(base, ResolvedChain::from(bits8!(0x5678).eq(1)));
    /// let resolved: ResolvedChain = pending.resolve();
    /// assert_eq!(
    ///     resolved,
    ///     ResolvedChain::from(vec![
    ///         bits8!(0x5678).eq(1),
    ///         bits8!(0x1234).eq(0),
    ///     ])
    /// );
    /// ```
    #[must_use]
    pub fn resolve(self) -> ResolvedChain {
        let mut chain = self.pending;
        chain.extend(self.head);
        chain
    }
}

impl Chain<MemoryRef> {
    /// Sets the access mode to [`AccessMode::Delta`][`crate::types::memory::AccessMode::Delta`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, ResolvedChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = Chain::new(bits8!(0x4321), chain).delta();
    /// assert_eq!(pending_chain.head(), &bits8!(0x4321).delta());
    /// ```
    #[must_use]
    pub fn delta(self) -> Self {
        Self {
            head: self.head.delta(),
            pending: self.pending,
        }
    }

    /// Sets the access mode to [`AccessMode::Prior`][`crate::types::memory::AccessMode::Prior`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, ResolvedChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = Chain::new(bits8!(0x4321), chain).prior();
    /// assert_eq!(pending_chain.head(), &bits8!(0x4321).prior());
    /// ```
    #[must_use]
    pub fn prior(self) -> Self {
        Self {
            head: self.head.prior(),
            pending: self.pending,
        }
    }

    /// Sets the access mode to [`AccessMode::BCD`][`crate::types::memory::AccessMode::BCD`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, ResolvedChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = Chain::new(bits8!(0x4321), chain).bcd();
    /// assert_eq!(pending_chain.head(), &bits8!(0x4321).bcd());
    /// ```
    #[must_use]
    pub fn bcd(self) -> Self {
        Self {
            head: self.head.bcd(),
            pending: self.pending,
        }
    }

    /// Sets the access mode to [`AccessMode::Invert`][`crate::types::memory::AccessMode::Invert`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, ResolvedChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = Chain::new(bits8!(0x4321), chain).invert();
    /// assert_eq!(pending_chain.head(), &bits8!(0x4321).invert());
    /// ```
    #[must_use]
    pub fn invert(self) -> Self {
        Self {
            head: self.head.invert(),
            pending: self.pending,
        }
    }
}

impl Chain<MemoryRef> {
    /// Sets the given arithmetic flag on the head memory reference.
    ///
    /// This converts the head from a [`MemoryRef`] into an [`Arithmetic`],
    /// returning a [`Chain<Arithmetic>`][ResolvedChain].
    #[must_use]
    pub fn with_arithmetic_flag(self, flag: ArithmeticFlag) -> Chain<Arithmetic> {
        Chain::new(self.head.with_flag(flag), self.pending)
    }
}

impl Measured for Chain<MemoryRef> {
    type Output = Chain<Arithmetic>;

    fn measured(self) -> Self::Output {
        let head = self.head;
        Chain::new(head.measured(), self.pending)
    }
}

impl_arithmetic_flag_traits!(MemoryChain, with_arithmetic_flag, Chain<Arithmetic>);

impl<T: Into<TypedValue> + Copy> Chain<T> {
    /// Extends the pending chain with an equals comparison.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, ResolvedChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = Chain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).eq(0)
    /// );
    /// assert_eq!(ResolvedChain::from(pending_chain.eq(0)), expected.into());
    /// ```
    pub fn eq(self, rhs: impl Into<TypedValue>) -> Chain<Condition> {
        let head = self.head;
        Chain::new(head.eq(rhs), self.pending)
    }

    /// Extends the pending chain with a not equals comparison.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, ResolvedChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = Chain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).ne(0)
    /// );
    /// assert_eq!(ResolvedChain::from(pending_chain.ne(0)), expected.into());
    /// ```
    pub fn ne(self, rhs: impl Into<TypedValue>) -> Chain<Condition> {
        let head = self.head;
        Chain::new(head.ne(rhs), self.pending)
    }

    /// Extends the pending chain with a less than comparison.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, ResolvedChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = Chain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).lt(0)
    /// );
    /// assert_eq!(ResolvedChain::from(pending_chain.lt(0)), expected.into());
    /// ```
    pub fn lt(self, rhs: impl Into<TypedValue>) -> Chain<Condition> {
        let head = self.head;
        Chain::new(head.lt(rhs), self.pending)
    }

    /// Extends the pending chain with a less than or equals comparison.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, ResolvedChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = Chain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).le(0)
    /// );
    /// assert_eq!(ResolvedChain::from(pending_chain.le(0)), expected.into());
    /// ```
    pub fn le(self, rhs: impl Into<TypedValue>) -> Chain<Condition> {
        let head = self.head;
        Chain::new(head.le(rhs), self.pending)
    }

    /// Extends the pending chain with a greater than comparison.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, ResolvedChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = Chain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).gt(0)
    /// );
    /// assert_eq!(ResolvedChain::from(pending_chain.gt(0)), expected.into());
    /// ```
    pub fn gt(self, rhs: impl Into<TypedValue>) -> Chain<Condition> {
        let head = self.head;
        Chain::new(head.gt(rhs), self.pending)
    }

    /// Extends the pending chain with a greater than or equals comparison.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, ResolvedChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = Chain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).ge(0)
    /// );
    /// assert_eq!(ResolvedChain::from(pending_chain.ge(0)), expected.into());
    /// ```
    pub fn ge(self, rhs: impl Into<TypedValue>) -> Chain<Condition> {
        let head = self.head;
        Chain::new(head.ge(rhs), self.pending)
    }

    /// Extends the pending chain with an addition operation.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, ResolvedChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = Chain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).add(0)
    /// );
    /// assert_eq!(ResolvedChain::from(pending_chain.add(0)), expected.into());
    /// ```
    #[expect(
        clippy::should_implement_trait,
        reason = "not using arithmetic in the traditional sense"
    )]
    pub fn add(self, rhs: impl Into<TypedValue>) -> Chain<Arithmetic> {
        let head = self.head;
        Chain::new(head.add(rhs), self.pending)
    }

    /// Extends the pending chain with a subtraction operation.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, ResolvedChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = Chain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).sub(0)
    /// );
    /// assert_eq!(ResolvedChain::from(pending_chain.sub(0)), expected.into());
    /// ```
    #[expect(
        clippy::should_implement_trait,
        reason = "not using arithmetic in the traditional sense"
    )]
    pub fn sub(self, rhs: impl Into<TypedValue>) -> Chain<Arithmetic> {
        let head = self.head;
        Chain::new(head.sub(rhs), self.pending)
    }

    /// Extends the pending chain with a multiplication operation.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, ResolvedChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = Chain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).mul(0)
    /// );
    /// assert_eq!(ResolvedChain::from(pending_chain.mul(0)), expected.into());
    /// ```
    #[expect(
        clippy::should_implement_trait,
        reason = "not using arithmetic in the traditional sense"
    )]
    pub fn mul(self, rhs: impl Into<TypedValue>) -> Chain<Arithmetic> {
        let head = self.head;
        Chain::new(head.mul(rhs), self.pending)
    }

    /// Extends the pending chain with a division operation.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, ResolvedChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = Chain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).div(0)
    /// );
    /// assert_eq!(ResolvedChain::from(pending_chain.div(0)), expected.into());
    /// ```
    #[expect(
        clippy::should_implement_trait,
        reason = "not using arithmetic in the traditional sense"
    )]
    pub fn div(self, rhs: impl Into<TypedValue>) -> Chain<Arithmetic> {
        let head = self.head;
        Chain::new(head.div(rhs), self.pending)
    }

    /// Extends the pending chain with a modulo operation.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, ResolvedChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = Chain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).modulo(0)
    /// );
    /// assert_eq!(ResolvedChain::from(pending_chain.modulo(0)), expected.into());
    /// ```
    pub fn modulo(self, rhs: impl Into<TypedValue>) -> Chain<Arithmetic> {
        let head = self.head;
        Chain::new(head.modulo(rhs), self.pending)
    }

    /// Extends the pending chain with a bitwise and operation.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, ResolvedChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = Chain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).bitwise_and(0)
    /// );
    /// assert_eq!(ResolvedChain::from(pending_chain.bitwise_and(0)), expected.into());
    /// ```
    pub fn bitwise_and(self, rhs: impl Into<TypedValue>) -> Chain<Arithmetic> {
        let head = self.head;
        Chain::new(head.bitwise_and(rhs), self.pending)
    }

    /// Extends the pending chain with a bitwise xor operation.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, ResolvedChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = Chain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).bitwise_xor(0)
    /// );
    /// assert_eq!(ResolvedChain::from(pending_chain.bitwise_xor(0)), expected.into());
    /// ```
    pub fn bitwise_xor(self, rhs: impl Into<TypedValue>) -> Chain<Arithmetic> {
        let head = self.head;
        Chain::new(head.bitwise_xor(rhs), self.pending)
    }
}

impl Chain<Condition> {
    /// Sets the hit count on the head condition.
    #[must_use]
    pub fn with_hits(self, hits: u32) -> Self {
        Self {
            head: self.head.with_hits(hits),
            pending: self.pending,
        }
    }

    /// Sets the given condition flag on the head condition.
    #[must_use]
    pub fn with_condition_flag(self, flag: ConditionFlag) -> Self {
        Self {
            head: self.head.with_flag(flag),
            pending: self.pending,
        }
    }
}

impl Chain<Arithmetic> {
    /// Sets the given arithmetic flag on the head arithmetic.
    #[must_use]
    pub fn with_arithmetic_flag(self, flag: ArithmeticFlag) -> Self {
        Self {
            head: self.head.with_flag(flag),
            pending: self.pending,
        }
    }
}

// Type aliases are required because `impl_condition_flag_traits!` and
// `impl_arithmetic_flag_traits!` expect a bare `$struct:ident`, not a
// generic type like `Chain<Condition>`.
#[allow(clippy::missing_docs_in_private_items)]
type ConditionChain = Chain<Condition>;
#[allow(clippy::missing_docs_in_private_items)]
type ArithmeticChain = Chain<Arithmetic>;
#[allow(clippy::missing_docs_in_private_items)]
type MemoryChain = Chain<MemoryRef>;

impl_condition_flag_traits!(ConditionChain, with_condition_flag);
impl_arithmetic_flag_traits!(ArithmeticChain, with_arithmetic_flag);

impl AccessModeModifier for Chain<Condition> {
    fn with_access_mode(self, access_mode: AccessMode) -> Self {
        Self {
            head: self.head.with_access_mode(access_mode),
            pending: self.pending,
        }
    }
}

impl AccessModeModifier for Chain<Arithmetic> {
    fn with_access_mode(self, access_mode: AccessMode) -> Self {
        Self {
            head: self.head.with_access_mode(access_mode),
            pending: self.pending,
        }
    }
}

impl<T: Into<Requirement>> From<Chain<T>> for ResolvedChain {
    fn from(pc: Chain<T>) -> Self {
        let mut chain = pc.pending;
        chain.extend(pc.head);
        chain
    }
}

impl From<Chain<ResolvedChain>> for ResolvedChain {
    fn from(pc: Chain<ResolvedChain>) -> Self {
        let mut chain = pc.pending;
        chain.extend(pc.head);
        chain
    }
}

impl Chainable for Requirement {
    type Output = Chain<Requirement>;

    fn chain(self, chain: ResolvedChain) -> Self::Output {
        Chain::new(self, chain)
    }
}

impl Chainable for ResolvedChain {
    type Output = Chain<ResolvedChain>;

    fn chain(self, chain: ResolvedChain) -> Self::Output {
        Chain::new(self, chain)
    }
}

impl Chainable for Condition {
    type Output = Chain<Condition>;

    fn chain(self, chain: ResolvedChain) -> Self::Output {
        Chain::new(self, chain)
    }
}

impl Chainable for Arithmetic {
    type Output = Chain<Arithmetic>;

    fn chain(self, chain: ResolvedChain) -> Self::Output {
        Chain::new(self, chain)
    }
}

impl Chainable for TypedValue {
    type Output = Chain<TypedValue>;

    fn chain(self, chain: ResolvedChain) -> Self::Output {
        Chain::new(self, chain)
    }
}

impl Chainable for MemoryRef {
    type Output = Chain<MemoryRef>;

    fn chain(self, chain: ResolvedChain) -> Self::Output {
        Chain::new(self, chain)
    }
}

impl<T: Chainable> Chainable for Chain<T> {
    type Output = Chain<T>;

    fn chain(self, chain: ResolvedChain) -> Self::Output {
        Chain::new(self.head, Chainable::chain(self.pending, chain))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::memory::{MemoryRef, MemorySize};

    fn condition_chain() -> ResolvedChain {
        ResolvedChain::from(vec![Condition::eq(0x1234, 0), Condition::eq(0x5678, 1)])
    }

    #[test]
    fn resolved_condition_head_folds_into_chain() {
        let pending = Chain::new(Condition::eq(0x1234, 0), ResolvedChain::new());
        let chain: ResolvedChain = pending.into();
        assert_eq!(chain, ResolvedChain::from(Condition::eq(0x1234, 0)));
    }

    #[test]
    fn nested_pending_chain_collapses_head_of_last_wins() {
        let mem = MemoryRef::new(MemorySize::Bits8, 0x5678);
        let mid = Chain::new(Condition::eq(0x1234, 0), ResolvedChain::new());
        let head = Chain::new(mem, ResolvedChain::new());

        let collapsed = Chainable::chain(head, ResolvedChain::from(mid));
        assert_eq!(collapsed.head(), &mem);

        let resolved: ResolvedChain = collapsed.eq(0).into();
        let expected: ResolvedChain =
            ResolvedChain::from(vec![Condition::eq(0x1234, 0), mem.eq(0)]);
        assert_eq!(resolved, expected);
    }

    #[test]
    fn mid_chain_pending_condition_extends_pending() {
        let pending = Chain::new(
            Condition::eq(0x5678, 1),
            ResolvedChain::from(Condition::eq(0x1234, 0)),
        );
        let resolved: ResolvedChain = pending.into();
        assert_eq!(resolved, condition_chain());
    }

    #[test]
    fn with_hits_on_pending_condition_keeps_pending() {
        let pending = Chain::new(Condition::eq(0x1234, 0), ResolvedChain::new()).with_hits(5);
        let resolved: ResolvedChain = pending.into();
        assert_eq!(
            resolved,
            ResolvedChain::from(Condition::eq(0x1234, 0).with_hits(5))
        );
    }
}

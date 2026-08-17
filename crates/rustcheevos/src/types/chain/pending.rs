//! Type definition for pending chains.

use crate::{
    impl_arithmetic_flag_traits, impl_condition_flag_traits,
    types::{
        chain::Chain,
        flag::{ArithmeticFlag, ConditionFlag, Measured},
        memory::{AccessMode, AccessModeModifier, MemoryRef},
        requirement::{Requirement, arithmetic::Arithmetic, condition::Condition},
        value::{TypedValue, TypedValueOps},
    },
};

/// A trait for types that can be chained in a [`Chain`].
pub trait Chainable {
    /// The output type.
    type Output;

    /// Chains the type with the given chain.
    fn chain(self, chain: Chain) -> Self::Output;
}

/// A pending chain of requirements.
///
/// This type is a specialized version of [`Chain`] that is used to build and compose chains
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
/// use rustcheevos::types::{chain::{Chain, PendingChain}, memory::MemoryRef};
/// use rustcheevos::{add_address, bits32, chain};
/// # impl Addr {
///
/// // Define a pending chain, with the head being a memory reference.
/// pub fn level(&self) -> PendingChain<MemoryRef> {
///     let offset = BASE_ADDR + *self as usize * 4;
///     chain!(
///         add_address!(current_profile().mul(PROFILE_STRIDE)),
///         bits32!(offset)
///     )
/// }
///
/// // The head of the chain can be modified to construct a new resolved chain.
/// pub fn is_level(&self, level: u32) -> Chain {
///     self.level().eq(level).into()
/// }
/// # }
#[derive(Debug)]
pub struct PendingChain<T> {
    /// The head of the chain.
    head: T,
    /// The pending chain.
    pending: Chain,
}

impl<T> PendingChain<T> {
    /// Creates a new pending chain.
    ///
    /// # Exampless
    ///
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, PendingChain};
    /// use rustcheevos::{add_address, bits32, chain};
    ///
    /// let chain = chain!(
    ///     add_address!(bits32!(0x1234)),
    ///     bits32!(0x5432).eq(0)
    /// );
    ///
    /// PendingChain::new(0, chain);
    pub fn new(head: T, pending: impl Into<Chain>) -> Self {
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
    /// use rustcheevos::types::chain::{Chain, PendingChain};
    /// use rustcheevos::{add_address, bits32, chain};
    ///
    /// let chain = chain!(
    ///     add_address!(bits32!(0x1234)),
    ///     bits32!(0x5432).eq(0)
    /// );
    ///
    /// let pending_chain = PendingChain::new(0, chain);
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
    /// use rustcheevos::types::chain::{Chain, PendingChain};
    /// use rustcheevos::{add_address, bits32, chain};
    ///
    /// let chain = chain!(bits32!(0x5432).eq(0));
    ///
    /// let pending_chain = PendingChain::new(0, chain);
    /// assert_eq!(pending_chain.pending(), &chain!(bits32!(0x5432).eq(0)));
    /// ```
    pub fn pending(&self) -> &Chain {
        &self.pending
    }
}

impl PendingChain<MemoryRef> {
    /// Sets the access mode to [`AccessMode::Delta`][`crate::types::memory::AccessMode::Delta`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, PendingChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain).delta();
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
    /// use rustcheevos::types::chain::{Chain, PendingChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain).prior();
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
    /// use rustcheevos::types::chain::{Chain, PendingChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain).bcd();
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
    /// use rustcheevos::types::chain::{Chain, PendingChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain).invert();
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

impl PendingChain<MemoryRef> {
    /// Sets the given arithmetic flag on the head memory reference.
    ///
    /// This converts the head from a [`MemoryRef`] into an [`Arithmetic`],
    /// returning a [`PendingChain<Arithmetic>`][PendingChain].
    #[must_use]
    pub fn with_arithmetic_flag(self, flag: ArithmeticFlag) -> PendingChain<Arithmetic> {
        PendingChain::new(self.head.with_flag(flag), self.pending)
    }
}

impl Measured for PendingChain<MemoryRef> {
    type Output = PendingChain<Arithmetic>;

    fn measured(self) -> Self::Output {
        let head = self.head;
        PendingChain::new(head.measured(), self.pending)
    }
}

impl_arithmetic_flag_traits!(
    PendingMemoryRef,
    with_arithmetic_flag,
    PendingChain<Arithmetic>
);

impl<T: Into<TypedValue> + Copy> PendingChain<T> {
    /// Extends the pending chain with an equals comparison.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, PendingChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).eq(0)
    /// );
    /// assert_eq!(Chain::from(pending_chain.eq(0)), expected);
    /// ```
    pub fn eq(self, rhs: impl Into<TypedValue>) -> PendingChain<Condition> {
        let head = self.head;
        PendingChain::new(head.eq(rhs), self.pending)
    }

    /// Extends the pending chain with a not equals comparison.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, PendingChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).ne(0)
    /// );
    /// assert_eq!(Chain::from(pending_chain.ne(0)), expected);
    /// ```
    pub fn ne(self, rhs: impl Into<TypedValue>) -> PendingChain<Condition> {
        let head = self.head;
        PendingChain::new(head.ne(rhs), self.pending)
    }

    /// Extends the pending chain with a less than comparison.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, PendingChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).lt(0)
    /// );
    /// assert_eq!(Chain::from(pending_chain.lt(0)), expected);
    /// ```
    pub fn lt(self, rhs: impl Into<TypedValue>) -> PendingChain<Condition> {
        let head = self.head;
        PendingChain::new(head.lt(rhs), self.pending)
    }

    /// Extends the pending chain with a less than or equals comparison.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, PendingChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).le(0)
    /// );
    /// assert_eq!(Chain::from(pending_chain.le(0)), expected);
    /// ```
    pub fn le(self, rhs: impl Into<TypedValue>) -> PendingChain<Condition> {
        let head = self.head;
        PendingChain::new(head.le(rhs), self.pending)
    }

    /// Extends the pending chain with a greater than comparison.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, PendingChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).gt(0)
    /// );
    /// assert_eq!(Chain::from(pending_chain.gt(0)), expected);
    /// ```
    pub fn gt(self, rhs: impl Into<TypedValue>) -> PendingChain<Condition> {
        let head = self.head;
        PendingChain::new(head.gt(rhs), self.pending)
    }

    /// Extends the pending chain with a greater than or equals comparison.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, PendingChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).ge(0)
    /// );
    /// assert_eq!(Chain::from(pending_chain.ge(0)), expected);
    /// ```
    pub fn ge(self, rhs: impl Into<TypedValue>) -> PendingChain<Condition> {
        let head = self.head;
        PendingChain::new(head.ge(rhs), self.pending)
    }

    /// Extends the pending chain with an addition operation.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, PendingChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).add(0)
    /// );
    /// assert_eq!(Chain::from(pending_chain.add(0)), expected);
    /// ```
    #[expect(
        clippy::should_implement_trait,
        reason = "not using arithmetic in the traditional sense"
    )]
    pub fn add(self, rhs: impl Into<TypedValue>) -> PendingChain<Arithmetic> {
        let head = self.head;
        PendingChain::new(head.add(rhs), self.pending)
    }

    /// Extends the pending chain with a subtraction operation.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, PendingChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).sub(0)
    /// );
    /// assert_eq!(Chain::from(pending_chain.sub(0)), expected);
    /// ```
    #[expect(
        clippy::should_implement_trait,
        reason = "not using arithmetic in the traditional sense"
    )]
    pub fn sub(self, rhs: impl Into<TypedValue>) -> PendingChain<Arithmetic> {
        let head = self.head;
        PendingChain::new(head.sub(rhs), self.pending)
    }

    /// Extends the pending chain with a multiplication operation.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, PendingChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).mul(0)
    /// );
    /// assert_eq!(Chain::from(pending_chain.mul(0)), expected);
    /// ```
    #[expect(
        clippy::should_implement_trait,
        reason = "not using arithmetic in the traditional sense"
    )]
    pub fn mul(self, rhs: impl Into<TypedValue>) -> PendingChain<Arithmetic> {
        let head = self.head;
        PendingChain::new(head.mul(rhs), self.pending)
    }

    /// Extends the pending chain with a division operation.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, PendingChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).div(0)
    /// );
    /// assert_eq!(Chain::from(pending_chain.div(0)), expected);
    /// ```
    #[expect(
        clippy::should_implement_trait,
        reason = "not using arithmetic in the traditional sense"
    )]
    pub fn div(self, rhs: impl Into<TypedValue>) -> PendingChain<Arithmetic> {
        let head = self.head;
        PendingChain::new(head.div(rhs), self.pending)
    }

    /// Extends the pending chain with a modulo operation.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, PendingChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).modulo(0)
    /// );
    /// assert_eq!(Chain::from(pending_chain.modulo(0)), expected);
    /// ```
    pub fn modulo(self, rhs: impl Into<TypedValue>) -> PendingChain<Arithmetic> {
        let head = self.head;
        PendingChain::new(head.modulo(rhs), self.pending)
    }

    /// Extends the pending chain with a bitwise and operation.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, PendingChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).bitwise_and(0)
    /// );
    /// assert_eq!(Chain::from(pending_chain.bitwise_and(0)), expected);
    /// ```
    pub fn bitwise_and(self, rhs: impl Into<TypedValue>) -> PendingChain<Arithmetic> {
        let head = self.head;
        PendingChain::new(head.bitwise_and(rhs), self.pending)
    }

    /// Extends the pending chain with a bitwise xor operation.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::{Chain, PendingChain};
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).bitwise_xor(0)
    /// );
    /// assert_eq!(Chain::from(pending_chain.bitwise_xor(0)), expected);
    /// ```
    pub fn bitwise_xor(self, rhs: impl Into<TypedValue>) -> PendingChain<Arithmetic> {
        let head = self.head;
        PendingChain::new(head.bitwise_xor(rhs), self.pending)
    }
}

impl PendingChain<Condition> {
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

impl PendingChain<Arithmetic> {
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
// generic type like `PendingChain<Condition>`.
#[allow(clippy::missing_docs_in_private_items)]
type PendingCondition = PendingChain<Condition>;
#[allow(clippy::missing_docs_in_private_items)]
type PendingArithmetic = PendingChain<Arithmetic>;
#[allow(clippy::missing_docs_in_private_items)]
type PendingMemoryRef = PendingChain<MemoryRef>;

impl_condition_flag_traits!(PendingCondition, with_condition_flag);
impl_arithmetic_flag_traits!(PendingArithmetic, with_arithmetic_flag);

impl AccessModeModifier for PendingChain<Condition> {
    fn with_access_mode(self, access_mode: AccessMode) -> Self {
        Self {
            head: self.head.with_access_mode(access_mode),
            pending: self.pending,
        }
    }
}

impl AccessModeModifier for PendingChain<Arithmetic> {
    fn with_access_mode(self, access_mode: AccessMode) -> Self {
        Self {
            head: self.head.with_access_mode(access_mode),
            pending: self.pending,
        }
    }
}

impl From<PendingChain<Condition>> for Chain {
    fn from(pc: PendingChain<Condition>) -> Self {
        let mut chain = pc.pending;
        chain.extend(pc.head);
        chain
    }
}

impl From<PendingChain<Arithmetic>> for Chain {
    fn from(pc: PendingChain<Arithmetic>) -> Self {
        let mut chain = pc.pending;
        chain.extend(pc.head);
        chain
    }
}

impl Chainable for Requirement {
    type Output = Chain;

    fn chain(self, mut chain: Chain) -> Self::Output {
        chain.extend(self);
        chain
    }
}

impl Chainable for Chain {
    type Output = Chain;

    fn chain(self, mut chain: Chain) -> Self::Output {
        chain.extend(self);
        chain
    }
}

impl Chainable for Condition {
    type Output = Chain;

    fn chain(self, mut chain: Chain) -> Self::Output {
        chain.extend(self);
        chain
    }
}

impl Chainable for Arithmetic {
    type Output = Chain;

    fn chain(self, mut chain: Chain) -> Self::Output {
        chain.extend(self);
        chain
    }
}

impl Chainable for TypedValue {
    type Output = PendingChain<TypedValue>;

    fn chain(self, chain: Chain) -> Self::Output {
        PendingChain::new(self, chain)
    }
}

impl Chainable for MemoryRef {
    type Output = PendingChain<MemoryRef>;

    fn chain(self, chain: Chain) -> Self::Output {
        PendingChain::new(self, chain)
    }
}

impl<T: Chainable> Chainable for PendingChain<T> {
    type Output = PendingChain<T>;

    fn chain(self, chain: Chain) -> Self::Output {
        PendingChain::new(self.head, Chainable::chain(self.pending, chain))
    }
}

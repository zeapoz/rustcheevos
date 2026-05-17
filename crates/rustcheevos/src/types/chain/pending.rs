//! Type definition for pending chains.

use crate::types::{
    chain::Chain,
    memory::{AccessModeModifier, MemoryRef},
    requirement::{Requirement, arithmetic::Arithmetic, condition::Condition},
    value::{TypedValue, TypedValueOps},
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
///     self.level().eq(level)
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
    /// use rustcheevos::types::chain::PendingChain;
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
    /// use rustcheevos::types::chain::PendingChain;
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
    /// use rustcheevos::types::chain::PendingChain;
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
    /// use rustcheevos::types::chain::PendingChain;
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
    /// use rustcheevos::types::chain::PendingChain;
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
    /// use rustcheevos::types::chain::PendingChain;
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
    /// use rustcheevos::types::chain::PendingChain;
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

impl<T: Into<TypedValue> + Copy> PendingChain<T> {
    /// Extends the pending chain with a requirement.
    fn extend_req(self, req: impl Into<Requirement>) -> Chain {
        let mut chain = self.pending;
        chain.extend(req);
        chain
    }

    /// Extends the pending chain with an equals comparison.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::PendingChain;
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).eq(0)
    /// );
    /// assert_eq!(pending_chain.eq(0), expected);
    /// ```
    pub fn eq(self, rhs: impl Into<TypedValue>) -> Chain {
        let head = self.head;
        self.extend_req(head.eq(rhs))
    }

    /// Extends the pending chain with a not equals comparison.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::PendingChain;
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).ne(0)
    /// );
    /// assert_eq!(pending_chain.ne(0), expected);
    /// ```
    pub fn ne(self, rhs: impl Into<TypedValue>) -> Chain {
        let head = self.head;
        self.extend_req(head.ne(rhs))
    }

    /// Extends the pending chain with a less than comparison.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::PendingChain;
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).lt(0)
    /// );
    /// assert_eq!(pending_chain.lt(0), expected);
    /// ```
    pub fn lt(self, rhs: impl Into<TypedValue>) -> Chain {
        let head = self.head;
        self.extend_req(head.lt(rhs))
    }

    /// Extends the pending chain with a less than or equals comparison.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::PendingChain;
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).le(0)
    /// );
    /// assert_eq!(pending_chain.le(0), expected);
    /// ```
    pub fn le(self, rhs: impl Into<TypedValue>) -> Chain {
        let head = self.head;
        self.extend_req(head.le(rhs))
    }

    /// Extends the pending chain with a greater than comparison.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::PendingChain;
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).gt(0)
    /// );
    /// assert_eq!(pending_chain.gt(0), expected);
    /// ```
    pub fn gt(self, rhs: impl Into<TypedValue>) -> Chain {
        let head = self.head;
        self.extend_req(head.gt(rhs))
    }

    /// Extends the pending chain with a greater than or equals comparison.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::PendingChain;
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).ge(0)
    /// );
    /// assert_eq!(pending_chain.ge(0), expected);
    /// ```
    pub fn ge(self, rhs: impl Into<TypedValue>) -> Chain {
        let head = self.head;
        self.extend_req(head.ge(rhs))
    }

    /// Extends the pending chain with an addition operation.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::PendingChain;
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).add(0)
    /// );
    /// assert_eq!(pending_chain.add(0), expected);
    /// ```
    #[expect(
        clippy::should_implement_trait,
        reason = "not using arithmetic in the traditional sense"
    )]
    pub fn add(self, rhs: impl Into<TypedValue>) -> Chain {
        let head = self.head;
        self.extend_req(head.add(rhs))
    }

    /// Extends the pending chain with a subtraction operation.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::PendingChain;
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).sub(0)
    /// );
    /// assert_eq!(pending_chain.sub(0), expected);
    /// ```
    #[expect(
        clippy::should_implement_trait,
        reason = "not using arithmetic in the traditional sense"
    )]
    pub fn sub(self, rhs: impl Into<TypedValue>) -> Chain {
        let head = self.head;
        self.extend_req(head.sub(rhs))
    }

    /// Extends the pending chain with a multiplication operation.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::PendingChain;
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).mul(0)
    /// );
    /// assert_eq!(pending_chain.mul(0), expected);
    /// ```
    #[expect(
        clippy::should_implement_trait,
        reason = "not using arithmetic in the traditional sense"
    )]
    pub fn mul(self, rhs: impl Into<TypedValue>) -> Chain {
        let head = self.head;
        self.extend_req(head.mul(rhs))
    }

    /// Extends the pending chain with a division operation.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::PendingChain;
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).div(0)
    /// );
    /// assert_eq!(pending_chain.div(0), expected);
    /// ```
    #[expect(
        clippy::should_implement_trait,
        reason = "not using arithmetic in the traditional sense"
    )]
    pub fn div(self, rhs: impl Into<TypedValue>) -> Chain {
        let head = self.head;
        self.extend_req(head.div(rhs))
    }

    /// Extends the pending chain with a modulo operation.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::PendingChain;
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).modulo(0)
    /// );
    /// assert_eq!(pending_chain.modulo(0), expected);
    /// ```
    pub fn modulo(self, rhs: impl Into<TypedValue>) -> Chain {
        let head = self.head;
        self.extend_req(head.modulo(rhs))
    }

    /// Extends the pending chain with a bitwise and operation.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::PendingChain;
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).bitwise_and(0)
    /// );
    /// assert_eq!(pending_chain.bitwise_and(0), expected);
    /// ```
    pub fn bitwise_and(self, rhs: impl Into<TypedValue>) -> Chain {
        let head = self.head;
        self.extend_req(head.bitwise_and(rhs))
    }

    /// Extends the pending chain with a bitwise xor operation.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::chain::PendingChain;
    /// use rustcheevos::{bits8, chain};
    ///
    /// let chain = chain!(bits8!(0x1234).eq(0));
    /// let pending_chain = PendingChain::new(bits8!(0x4321), chain);
    ///
    /// let expected = chain!(
    ///     bits8!(0x1234).eq(0),
    ///     bits8!(0x4321).bitwise_xor(0)
    /// );
    /// assert_eq!(pending_chain.bitwise_xor(0), expected);
    /// ```
    pub fn bitwise_xor(self, rhs: impl Into<TypedValue>) -> Chain {
        let head = self.head;
        self.extend_req(head.bitwise_xor(rhs))
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

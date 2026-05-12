use crate::{
    prelude::{MemoryRef, Requirement},
    types::requirement::{arithmetic::ArithmeticRequirement, comparison::ComparisonRequirement},
    types::value::TypedValue,
};

use super::Chain;

pub trait Chainable {
    type Output;

    fn chain(self, chain: Chain) -> Self::Output;
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

impl Chainable for ComparisonRequirement {
    type Output = Chain;

    fn chain(self, mut chain: Chain) -> Self::Output {
        chain.extend(self);
        chain
    }
}

impl Chainable for ArithmeticRequirement {
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
    type Output = PendingChain<TypedValue>;

    fn chain(self, chain: Chain) -> Self::Output {
        PendingChain::new(self.memory(), chain)
    }
}

impl Chainable for PendingChain<TypedValue> {
    type Output = PendingChain<TypedValue>;

    fn chain(self, chain: Chain) -> Self::Output {
        PendingChain::new(self.head, self.pending.chain(chain))
    }
}

#[derive(Debug)]
pub struct PendingChain<T> {
    head: T,
    pending: Chain,
}

impl<T> PendingChain<T> {
    pub fn new(head: T, pending: impl Into<Chain>) -> Self {
        Self {
            head,
            pending: pending.into(),
        }
    }
}

impl PendingChain<TypedValue> {
    fn extend_req(self, req: impl Into<Requirement>) -> Chain {
        let mut chain = self.pending;
        chain.extend(req);
        chain
    }

    pub fn eq(self, rhs: impl Into<TypedValue>) -> Chain {
        let head = self.head;
        self.extend_req(head.eq(rhs))
    }

    pub fn ne(self, rhs: impl Into<TypedValue>) -> Chain {
        let head = self.head;
        self.extend_req(head.ne(rhs))
    }

    pub fn lt(self, rhs: impl Into<TypedValue>) -> Chain {
        let head = self.head;
        self.extend_req(head.lt(rhs))
    }

    pub fn le(self, rhs: impl Into<TypedValue>) -> Chain {
        let head = self.head;
        self.extend_req(head.le(rhs))
    }

    pub fn gt(self, rhs: impl Into<TypedValue>) -> Chain {
        let head = self.head;
        self.extend_req(head.gt(rhs))
    }

    pub fn ge(self, rhs: impl Into<TypedValue>) -> Chain {
        let head = self.head;
        self.extend_req(head.ge(rhs))
    }

    pub fn add(self, rhs: impl Into<TypedValue>) -> Chain {
        let head = self.head;
        self.extend_req(head.add(rhs))
    }

    pub fn sub(self, rhs: impl Into<TypedValue>) -> Chain {
        let head = self.head;
        self.extend_req(head.sub(rhs))
    }

    pub fn mul(self, rhs: impl Into<TypedValue>) -> Chain {
        let head = self.head;
        self.extend_req(head.mul(rhs))
    }

    pub fn div(self, rhs: impl Into<TypedValue>) -> Chain {
        let head = self.head;
        self.extend_req(head.div(rhs))
    }

    pub fn modulo(self, rhs: impl Into<TypedValue>) -> Chain {
        let head = self.head;
        self.extend_req(head.modulo(rhs))
    }

    pub fn bitwise_and(self, rhs: impl Into<TypedValue>) -> Chain {
        let head = self.head;
        self.extend_req(head.bitwise_and(rhs))
    }

    pub fn bitwise_xor(self, rhs: impl Into<TypedValue>) -> Chain {
        let head = self.head;
        self.extend_req(head.bitwise_xor(rhs))
    }

    pub fn delta(self) -> Self {
        Self {
            head: self.head.delta(),
            pending: self.pending,
        }
    }

    pub fn prior(self) -> Self {
        Self {
            head: self.head.prior(),
            pending: self.pending,
        }
    }

    pub fn bcd(self) -> Self {
        Self {
            head: self.head.bcd(),
            pending: self.pending,
        }
    }

    pub fn invert(self) -> Self {
        Self {
            head: self.head.invert(),
            pending: self.pending,
        }
    }
}

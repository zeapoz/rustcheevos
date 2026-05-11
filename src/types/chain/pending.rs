use crate::{
    prelude::{MemoryRef, Requirement},
    types::value::TypedValue,
};

use super::Chain;

pub trait Chainable {
    type Output;

    fn chain(self, chain: Chain) -> Self::Output;
}

impl<T: Into<Chain>> Chainable for T {
    type Output = Chain;

    fn chain(self, mut chain: Chain) -> Self::Output {
        chain.extend(self.into());
        chain
    }
}

impl Chainable for MemoryRef {
    type Output = PendingChain<MemoryRef>;

    fn chain(self, chain: Chain) -> Self::Output {
        PendingChain::new(self, chain)
    }
}

impl Chainable for PendingChain<MemoryRef> {
    type Output = PendingChain<MemoryRef>;

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

impl PendingChain<MemoryRef> {
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
}

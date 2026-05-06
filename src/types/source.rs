//! Source and operation types for conditions.

use crate::types::memory::{MemOrValue, MemoryType};

/// The memory source for a condition.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Source {
    /// The memory reference or literal value.
    pub reference: MemOrValue,
    /// Optional flag modifying the condition.
    pub flag: Option<super::flag::Flag>,
    /// Optional memory type modifier.
    pub memtype: Option<MemoryType>,
}

/// An operation to apply to the source value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Operation {
    /// The comparison operator.
    pub op: super::operator::Operator,
    /// The target value to compare against.
    pub target: MemOrValue,
}

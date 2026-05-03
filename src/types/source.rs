use crate::types::memory::{MemOrValue, MemoryType};

#[derive(Clone, Debug, PartialEq)]
pub struct Source {
    pub reference: MemOrValue,
    pub flag: Option<super::flag::Flag>,
    pub memtype: Option<MemoryType>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Operation {
    pub op: super::operator::Operator,
    pub target: MemOrValue,
}

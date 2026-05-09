use std::{fmt, rc::Rc};

use builtin::BuiltInMacro;

use crate::prelude::{MemoryRef, RequirementGroup};

use super::{format::Format, lookup::LookupTable};

pub mod builtin;

/// A rich presence macro call.
#[derive(Debug, Clone, PartialEq)]
pub struct MacroRef {
    pub macro_type: MacroTypeRef,
    pub value: MacroValue,
}

impl MacroRef {
    /// Returns a new [`MacroHandle`] with the given format and value.
    pub fn new(macro_type: MacroTypeRef, value: impl Into<MacroValue>) -> Self {
        Self {
            macro_type,
            value: value.into(),
        }
    }

    /// Returns a new [`MacroHandle`] for a builtin macro.
    pub fn builtin(builtin: BuiltInMacro, value: impl Into<MacroValue>) -> MacroRef {
        Self::new(MacroTypeRef::Builtin(builtin), value)
    }

    /// Returns a new [`MacroHandle`] for a format.
    pub fn format(format: Rc<Format>, value: impl Into<MacroValue>) -> MacroRef {
        Self::new(MacroTypeRef::Format(format), value)
    }

    /// Returns a new [`MacroHandle`] for a lookup table.
    pub fn lookup(lookup: Rc<LookupTable>, value: impl Into<MacroValue>) -> MacroRef {
        Self::new(MacroTypeRef::Lookup(lookup), value)
    }
}

impl fmt::Display for MacroRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{}({})", self.macro_type, self.value)
    }
}

/// References to available macro types.
#[derive(Debug, Clone, PartialEq)]
pub enum MacroTypeRef {
    Builtin(BuiltInMacro),
    Format(Rc<Format>),
    Lookup(Rc<LookupTable>),
}

impl fmt::Display for MacroTypeRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Builtin(builtin) => builtin.to_string(),
            Self::Format(format) => format.name.to_string(),
            Self::Lookup(lookup) => lookup.name.to_string(),
        };
        write!(f, "{s}")
    }
}

/// A rich presence macro value.
#[derive(Debug, Clone, PartialEq)]
pub enum MacroValue {
    Memory(MemoryRef),
    Arithmetic(RequirementGroup),
}

impl From<MemoryRef> for MacroValue {
    fn from(value: MemoryRef) -> Self {
        Self::Memory(value)
    }
}

impl From<RequirementGroup> for MacroValue {
    fn from(value: RequirementGroup) -> Self {
        Self::Arithmetic(value)
    }
}

impl fmt::Display for MacroValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Memory(memory) => memory.to_string(),
            Self::Arithmetic(arithmetic) => arithmetic.to_string(),
        };
        write!(f, "{}", s)
    }
}

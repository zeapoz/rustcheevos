//! Type definitions for rich presence macros.

use std::{fmt, rc::Rc};

use builtin::BuiltInMacro;

use crate::types::{chain::Chain, memory::MemoryRef};

use super::{format::Format, lookup::LookupTable};

pub(crate) mod builtin;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum MacroType {
    /// A builtin macro (e.g., Number, Score, Seconds).
    Builtin(BuiltInMacro),
    /// A custom format reference.
    Format(Rc<Format>),
    /// A lookup table reference.
    Lookup(Rc<LookupTable>),
}

impl fmt::Display for MacroType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Builtin(builtin) => builtin.to_string(),
            Self::Format(format) => format.name().to_string(),
            Self::Lookup(lookup) => lookup.name().to_string(),
        };
        write!(f, "{s}")
    }
}

/// A rich presence macro reference.
#[derive(Debug, Clone, PartialEq)]
pub struct MacroRef {
    /// The type of macro being referenced.
    macro_type: MacroType,
    /// The macro value.
    value: MacroValue,
}

impl MacroRef {
    /// Creates a new macro reference.
    pub(crate) fn new(macro_type: MacroType, value: impl Into<MacroValue>) -> Self {
        Self {
            macro_type,
            value: value.into(),
        }
    }

    /// Creates a new builtin macro reference.
    pub fn builtin(builtin: BuiltInMacro, value: impl Into<MacroValue>) -> MacroRef {
        Self::new(MacroType::Builtin(builtin), value)
    }

    /// Creates a new format macro reference.
    pub(crate) fn format(format: Rc<Format>, value: impl Into<MacroValue>) -> MacroRef {
        Self::new(MacroType::Format(format), value)
    }

    /// Creates a new lookup macro reference.
    pub fn lookup(lookup: Rc<LookupTable>, value: impl Into<MacroValue>) -> MacroRef {
        Self::new(MacroType::Lookup(lookup), value)
    }
}

impl fmt::Display for MacroRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{}({})", self.macro_type, self.value)
    }
}

/// A rich presence macro value.
#[derive(Debug, Clone, PartialEq)]
pub enum MacroValue {
    /// A memory reference value.
    Memory(MemoryRef),
    /// An arithmetic chain value.
    Arithmetic(Chain),
}

impl From<MemoryRef> for MacroValue {
    fn from(value: MemoryRef) -> Self {
        Self::Memory(value)
    }
}

impl<T: Into<Chain>> From<T> for MacroValue {
    fn from(value: T) -> Self {
        Self::Arithmetic(value.into())
    }
}

impl fmt::Display for MacroValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Memory(memory) => memory.to_string(),
            Self::Arithmetic(arithmetic) => arithmetic.to_string(),
        };
        write!(f, "{s}")
    }
}

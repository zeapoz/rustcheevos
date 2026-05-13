use std::{fmt, rc::Rc};

use builtin::BuiltInMacro;

use crate::prelude::{Chain, MemoryRef};

use super::{format::Format, lookup::LookupTable};

pub mod builtin;

/// A rich presence macro call.
#[derive(Debug, Clone, PartialEq)]
pub struct MacroCall {
    pub macro_type: MacroRef,
    pub value: MacroValue,
}

impl MacroCall {
    /// Returns a new [`MacroCall`] with the given format and value.
    pub fn new(macro_type: MacroRef, value: impl Into<MacroValue>) -> Self {
        Self {
            macro_type,
            value: value.into(),
        }
    }

    /// Returns a new [`MacroCall`] for a builtin macro.
    pub fn builtin(builtin: BuiltInMacro, value: impl Into<MacroValue>) -> MacroCall {
        Self::new(MacroRef::Builtin(builtin), value)
    }

    /// Returns a new [`MacroCall`] for a format.
    pub fn format(format: Rc<Format>, value: impl Into<MacroValue>) -> MacroCall {
        Self::new(MacroRef::Format(format), value)
    }

    /// Returns a new [`MacroCall`] for a lookup table.
    pub fn lookup(lookup: Rc<LookupTable>, value: impl Into<MacroValue>) -> MacroCall {
        Self::new(MacroRef::Lookup(lookup), value)
    }
}

impl fmt::Display for MacroCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{}({})", self.macro_type, self.value)
    }
}

/// References to available macro types.
#[derive(Debug, Clone, PartialEq)]
pub enum MacroRef {
    Builtin(BuiltInMacro),
    Format(Rc<Format>),
    Lookup(Rc<LookupTable>),
}

impl MacroRef {
    /// Returns a builtin macro reference.
    pub fn builtin(builtin: BuiltInMacro) -> Self {
        Self::Builtin(builtin)
    }

    /// Returns a format reference.
    pub fn format(format: Rc<Format>) -> Self {
        Self::Format(format)
    }

    /// Returns a lookup table reference.
    pub fn lookup(lookup: Rc<LookupTable>) -> Self {
        Self::Lookup(lookup)
    }

    /// Calls the referenced macro with the given value to generate a [`MacroCall`]. Calls can be
    /// used directly within a format string.
    ///
    /// # Arguments
    /// * `value` - The value to pass to the macro.
    pub fn call(&self, value: impl Into<MacroValue>) -> MacroCall {
        MacroCall::new(self.clone(), value)
    }
}

impl fmt::Display for MacroRef {
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

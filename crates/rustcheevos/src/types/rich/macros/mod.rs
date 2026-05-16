//! Type definitions for rich presence macros.

use std::{fmt, rc::Rc};

use builtin::BuiltInMacro;

use crate::prelude::{Chain, MemoryRef};

use super::{format::Format, lookup::LookupTable};

pub mod builtin;

/// The type of macro being referenced.
#[derive(Debug, Clone, PartialEq)]
pub enum MacroType {
    /// A builtin macro (e.g., Number, Score, Seconds).
    Builtin(BuiltInMacro),
    /// A custom format reference.
    Format(Rc<Format>),
    /// A lookup table reference.
    Lookup(Rc<LookupTable>),
}

impl MacroType {
    /// Creates a builtin macro type.
    #[must_use]
    pub fn builtin(builtin: BuiltInMacro) -> Self {
        Self::Builtin(builtin)
    }

    /// Creates a format macro type.
    #[must_use]
    pub fn format(format: Rc<Format>) -> Self {
        Self::Format(format)
    }

    /// Creates a lookup macro type.
    #[must_use]
    pub fn lookup(lookup: Rc<LookupTable>) -> Self {
        Self::Lookup(lookup)
    }
}

impl fmt::Display for MacroType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Builtin(builtin) => builtin.to_string(),
            Self::Format(format) => format.name.to_string(),
            Self::Lookup(lookup) => lookup.name.to_string(),
        };
        write!(f, "{s}")
    }
}

/// A rich presence macro reference.
#[derive(Debug, Clone, PartialEq)]
pub struct MacroRef {
    /// The type of macro being referenced.
    pub macro_type: MacroType,
    /// The macro value.
    pub value: MacroValue,
}

impl MacroRef {
    /// Creates a new macro reference.
    #[must_use]
    pub fn new(macro_type: MacroType, value: impl Into<MacroValue>) -> Self {
        Self {
            macro_type,
            value: value.into(),
        }
    }

    /// Creates a new builtin macro reference.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::bits8;
    /// use rustcheevos::types::rich::macros::{builtin::BuiltInMacro, MacroRef};
    ///
    /// let macro_ref = MacroRef::builtin(BuiltInMacro::Number, bits8!(0x1234));
    /// assert_eq!(macro_ref.to_string(), "@Number(0xH1234)");
    /// ```
    pub fn builtin(builtin: BuiltInMacro, value: impl Into<MacroValue>) -> MacroRef {
        Self::new(MacroType::Builtin(builtin), value)
    }

    /// Creates a new format macro reference.
    ///
    /// # Examples
    /// ```
    /// use std::rc::Rc;
    /// use rustcheevos::bits8;
    /// use rustcheevos::types::rich::{macros::MacroRef, format::{Format, FormatType}};
    ///
    /// let format = Rc::new(Format::new("CustomFormat", FormatType::Value));
    /// let macro_ref = MacroRef::format(format, bits8!(0x1234));
    /// assert_eq!(macro_ref.to_string(), "@CustomFormat(0xH1234)");
    /// ```
    pub fn format(format: Rc<Format>, value: impl Into<MacroValue>) -> MacroRef {
        Self::new(MacroType::Format(format), value)
    }

    /// Creates a new lookup macro reference.
    ///
    /// # Examples
    /// ```
    /// use std::rc::Rc;
    /// use rustcheevos::bits8;
    /// use rustcheevos::types::rich::{lookup::LookupTable, macros::MacroRef};
    ///
    /// let lookup = Rc::new(LookupTable::new("LookupTable"));
    /// let macro_ref = MacroRef::lookup(lookup, bits8!(0x1234));
    /// assert_eq!(macro_ref.to_string(), "@LookupTable(0xH1234)");
    /// ```
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

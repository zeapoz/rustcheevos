//! Type definitions for rich presence macros.

use std::{fmt, rc::Rc};

use builtin::BuiltInMacro;

use crate::prelude::{Chain, MemoryRef};

use super::{format::Format, lookup::LookupTable};

pub mod builtin;

/// A rich presence macro call.
#[derive(Debug, Clone, PartialEq)]
pub struct MacroCall {
    /// The macro reference (builtin, format, or lookup).
    pub macro_type: MacroRef,
    /// The value to pass to the macro.
    pub value: MacroValue,
}

impl MacroCall {
    /// Creates a new macro call with the given macro reference and value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::{prelude::*, bits8};
    /// use rustcheevos::types::rich::macros::{MacroCall, MacroRef, builtin::BuiltInMacro};
    ///
    /// let call = MacroCall::new(
    ///     MacroRef::Builtin(BuiltInMacro::Score),
    ///     bits8!(0x1234),
    /// );
    /// assert_eq!(call.to_string(), "@Score(0xH1234)");
    /// ```
    pub fn new(macro_type: MacroRef, value: impl Into<MacroValue>) -> Self {
        Self {
            macro_type,
            value: value.into(),
        }
    }

    /// Creates a new macro call for a builtin macro.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::{prelude::*, bits8};
    /// use rustcheevos::types::rich::macros::{MacroCall, builtin::BuiltInMacro};
    ///
    /// let call = MacroCall::builtin(BuiltInMacro::Score, bits8!(0x1234));
    /// assert_eq!(call.to_string(), "@Score(0xH1234)");
    /// ```
    pub fn builtin(builtin: BuiltInMacro, value: impl Into<MacroValue>) -> MacroCall {
        Self::new(MacroRef::Builtin(builtin), value)
    }

    /// Creates a new macro call for a custom format.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use rustcheevos::{prelude::*, bits8};
    /// use rustcheevos::types::rich::{format::Format, format::FormatType, macros::MacroCall};
    ///
    /// let format = Rc::new(Format::new("Score".to_string(), FormatType::Score));
    /// let call = MacroCall::format(format, bits8!(0x1234));
    /// assert_eq!(call.to_string(), "@Score(0xH1234)");
    /// ```
    pub fn format(format: Rc<Format>, value: impl Into<MacroValue>) -> MacroCall {
        Self::new(MacroRef::Format(format), value)
    }

    /// Creates a new macro call for a lookup table.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use rustcheevos::{prelude::*, bits8};
    /// use rustcheevos::types::rich::{lookup::LookupTable, macros::MacroCall};
    ///
    /// let lookup = Rc::new(LookupTable::new("Health".to_string()));
    /// let call = MacroCall::lookup(lookup, bits8!(0x1234));
    /// assert_eq!(call.to_string(), "@Health(0xH1234)");
    /// ```
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
    /// A builtin macro (e.g., Number, Score, Seconds).
    Builtin(BuiltInMacro),
    /// A custom format reference.
    Format(Rc<Format>),
    /// A lookup table reference.
    Lookup(Rc<LookupTable>),
}

impl MacroRef {
    /// Creates a builtin macro reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::types::rich::macros::{MacroRef, builtin::BuiltInMacro};
    ///
    /// let ref_ = MacroRef::builtin(BuiltInMacro::Score);
    /// assert_eq!(ref_.to_string(), "Score");
    /// ```
    #[must_use]
    pub fn builtin(builtin: BuiltInMacro) -> Self {
        Self::Builtin(builtin)
    }

    /// Creates a format reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use rustcheevos::types::rich::{format::Format, format::FormatType, macros::MacroRef};
    ///
    /// let format = Rc::new(Format::new("Score".to_string(), FormatType::Score));
    /// let ref_ = MacroRef::format(format);
    /// assert_eq!(ref_.to_string(), "Score");
    /// ```
    #[must_use]
    pub fn format(format: Rc<Format>) -> Self {
        Self::Format(format)
    }

    /// Creates a lookup table reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use rustcheevos::types::rich::{lookup::LookupTable, macros::MacroRef};
    ///
    /// let lookup = Rc::new(LookupTable::new("Health".to_string()));
    /// let ref_ = MacroRef::lookup(lookup);
    /// assert_eq!(ref_.to_string(), "Health");
    /// ```
    #[must_use]
    pub fn lookup(lookup: Rc<LookupTable>) -> Self {
        Self::Lookup(lookup)
    }

    /// Calls the referenced macro with the given value to generate a [`MacroCall`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::{prelude::*, bits16};
    /// use rustcheevos::types::rich::macros::{MacroRef, builtin::BuiltInMacro};
    ///
    /// let ref_ = MacroRef::builtin(BuiltInMacro::Number);
    /// let call = ref_.call(bits16!(0x1234));
    /// assert_eq!(call.to_string(), "@Number(0x 1234)");
    /// ```
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

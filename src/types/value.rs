//! Type definitions for values.

use std::{fmt, str::FromStr};

use winnow::Parser;

use crate::{impl_arithmetic_flag_traits, parsers::ParseError, parsers::parse_typed_value};

use super::{
    flag::ArithmeticFlag,
    memory::MemoryRef,
    requirement::{arithmetic::ArithmeticRequirement, comparison::ComparisonRequirement},
};

/// A value that can be used in a condition.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypedValue {
    /// A memory reference value.
    Memory(MemoryRef),
    /// An integer value.
    Integer(u32),
    /// A float value.
    Float(f32),
    /// A recalled value.
    Recall,
}

impl TypedValue {
    /// Creates a new arithmetic [`ArithmeticRequirement`].
    #[must_use]
    pub fn with_arithmetic_flag(self, flag: ArithmeticFlag) -> ArithmeticRequirement {
        ArithmeticRequirement::new(flag, self)
    }
}

/// Operations for types that can be converted to [`TypedValue`].
pub trait TypedValueOps: Into<TypedValue> {
    /// Creates an equals [`ComparisonRequirement`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// let requirement = memory_ref.eq(50);
    /// ```
    fn eq(self, rhs: impl Into<TypedValue>) -> ComparisonRequirement;

    /// Creates a not equals [`ComparisonRequirement`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// let requirement = memory_ref.ne(50);
    /// ```
    fn ne(self, rhs: impl Into<TypedValue>) -> ComparisonRequirement;

    /// Creates a less than [`ComparisonRequirement`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// let requirement = memory_ref.lt(50);
    /// ```
    fn lt(self, rhs: impl Into<TypedValue>) -> ComparisonRequirement;

    /// Creates a less than or equals [`ComparisonRequirement`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// let requirement = memory_ref.le(50);
    /// ```
    fn le(self, rhs: impl Into<TypedValue>) -> ComparisonRequirement;

    /// Creates a greater than [`ComparisonRequirement`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// let requirement = memory_ref.gt(50);
    /// ```
    fn gt(self, rhs: impl Into<TypedValue>) -> ComparisonRequirement;

    /// Creates a greater than or equals [`ComparisonRequirement`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// let requirement = memory_ref.ge(50);
    /// ```
    fn ge(self, rhs: impl Into<TypedValue>) -> ComparisonRequirement;

    /// Creates an add [`ArithmeticRequirement`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// let requirement = memory_ref.add(50);
    /// ```
    fn add(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement;

    /// Creates a subtract [`ArithmeticRequirement`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// let requirement = memory_ref.sub(50);
    /// ```
    fn sub(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement;

    /// Creates a multiply [`ArithmeticRequirement`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// let requirement = memory_ref.mul(50);
    /// ```
    fn mul(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement;

    /// Creates a divide [`ArithmeticRequirement`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// let requirement = memory_ref.div(50);
    /// ```
    fn div(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement;

    /// Creates a modulo [`ArithmeticRequirement`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// let requirement = memory_ref.modulo(50);
    /// ```
    fn modulo(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement;

    /// Creates a bitwise and [`ArithmeticRequirement`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// let requirement = memory_ref.bitwise_and(50);
    /// ```
    fn bitwise_and(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement;

    /// Creates a bitwise xor [`ArithmeticRequirement`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// let requirement = memory_ref.bitwise_xor(50);
    /// ```
    fn bitwise_xor(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement;
}

impl<T: Into<TypedValue>> TypedValueOps for T {
    fn eq(self, rhs: impl Into<TypedValue>) -> ComparisonRequirement {
        ComparisonRequirement::eq(self, rhs)
    }

    fn ne(self, rhs: impl Into<TypedValue>) -> ComparisonRequirement {
        ComparisonRequirement::ne(self, rhs)
    }

    fn lt(self, rhs: impl Into<TypedValue>) -> ComparisonRequirement {
        ComparisonRequirement::lt(self, rhs)
    }

    fn le(self, rhs: impl Into<TypedValue>) -> ComparisonRequirement {
        ComparisonRequirement::le(self, rhs)
    }

    fn gt(self, rhs: impl Into<TypedValue>) -> ComparisonRequirement {
        ComparisonRequirement::gt(self, rhs)
    }

    fn ge(self, rhs: impl Into<TypedValue>) -> ComparisonRequirement {
        ComparisonRequirement::ge(self, rhs)
    }

    fn add(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement {
        ArithmeticRequirement::new(ArithmeticFlag::default(), self).add(rhs)
    }

    fn sub(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement {
        ArithmeticRequirement::new(ArithmeticFlag::default(), self).sub(rhs)
    }

    fn mul(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement {
        ArithmeticRequirement::new(ArithmeticFlag::default(), self).mul(rhs)
    }

    fn div(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement {
        ArithmeticRequirement::new(ArithmeticFlag::default(), self).div(rhs)
    }

    fn modulo(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement {
        ArithmeticRequirement::new(ArithmeticFlag::default(), self).modulo(rhs)
    }

    fn bitwise_and(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement {
        ArithmeticRequirement::new(ArithmeticFlag::default(), self).bitwise_and(rhs)
    }

    fn bitwise_xor(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement {
        ArithmeticRequirement::new(ArithmeticFlag::default(), self).bitwise_xor(rhs)
    }
}

impl From<u32> for TypedValue {
    fn from(value: u32) -> Self {
        TypedValue::Integer(value)
    }
}

impl From<f32> for TypedValue {
    fn from(value: f32) -> Self {
        TypedValue::Float(value)
    }
}

impl From<MemoryRef> for TypedValue {
    fn from(value: MemoryRef) -> Self {
        TypedValue::Memory(value)
    }
}

impl FromStr for TypedValue {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_typed_value
            .parse(s)
            .map_err(|s| ParseError::Value(s.to_string()))
    }
}

impl fmt::Display for TypedValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TypedValue::Memory(memory) => memory.to_string(),
            TypedValue::Integer(value) => value.to_string(),
            TypedValue::Float(value) => format!("f{value}"),
            TypedValue::Recall => "{recall}".to_string(),
        };
        write!(f, "{s}")
    }
}

impl_arithmetic_flag_traits!(TypedValue, with_arithmetic_flag, ArithmeticRequirement);

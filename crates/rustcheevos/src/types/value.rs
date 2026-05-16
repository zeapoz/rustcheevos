//! Type definitions for values.

use std::{fmt, str::FromStr};

use winnow::Parser;

use crate::{impl_arithmetic_flag_traits, parsers::ParseError, parsers::parse_typed_value};

use super::{
    flag::ArithmeticFlag,
    memory::{AccessMode, MemoryRef},
    requirement::{arithmetic::Arithmetic, condition::Condition},
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
    /// Creates a new arithmetic [`Arithmetic`].
    #[must_use]
    pub fn with_arithmetic_flag(self, flag: ArithmeticFlag) -> Arithmetic {
        Arithmetic::new(flag, self, None)
    }

    /// Applies an access mode if this value contains a [`MemoryRef`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let value = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// let typed = TypedValue::from(value).with_access_mode(AccessMode::Delta);
    /// assert_eq!(typed, TypedValue::from(MemoryRef::new(MemorySize::Bits8, 0x1234).delta()));
    /// ```
    #[must_use]
    pub fn with_access_mode(mut self, access_mode: AccessMode) -> Self {
        if let TypedValue::Memory(ref mut mem) = self {
            *mem = mem.with_access_mode(access_mode);
        }
        self
    }
}

/// Operations for types that can be converted to [`TypedValue`].
pub trait TypedValueOps: Into<TypedValue> {
    /// Creates an equals [`Condition`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// let requirement = memory_ref.eq(50);
    /// ```
    fn eq(self, rhs: impl Into<TypedValue>) -> Condition;

    /// Creates a not equals [`Condition`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// let requirement = memory_ref.ne(50);
    /// ```
    fn ne(self, rhs: impl Into<TypedValue>) -> Condition;

    /// Creates a less than [`Condition`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// let requirement = memory_ref.lt(50);
    /// ```
    fn lt(self, rhs: impl Into<TypedValue>) -> Condition;

    /// Creates a less than or equals [`Condition`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// let requirement = memory_ref.le(50);
    /// ```
    fn le(self, rhs: impl Into<TypedValue>) -> Condition;

    /// Creates a greater than [`Condition`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// let requirement = memory_ref.gt(50);
    /// ```
    fn gt(self, rhs: impl Into<TypedValue>) -> Condition;

    /// Creates a greater than or equals [`Condition`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// let requirement = memory_ref.ge(50);
    /// ```
    fn ge(self, rhs: impl Into<TypedValue>) -> Condition;

    /// Creates an add [`Arithmetic`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// let requirement = memory_ref.add(50);
    /// ```
    fn add(self, rhs: impl Into<TypedValue>) -> Arithmetic;

    /// Creates a subtract [`Arithmetic`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// let requirement = memory_ref.sub(50);
    /// ```
    fn sub(self, rhs: impl Into<TypedValue>) -> Arithmetic;

    /// Creates a multiply [`Arithmetic`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// let requirement = memory_ref.mul(50);
    /// ```
    fn mul(self, rhs: impl Into<TypedValue>) -> Arithmetic;

    /// Creates a divide [`Arithmetic`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// let requirement = memory_ref.div(50);
    /// ```
    fn div(self, rhs: impl Into<TypedValue>) -> Arithmetic;

    /// Creates a modulo [`Arithmetic`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// let requirement = memory_ref.modulo(50);
    /// ```
    fn modulo(self, rhs: impl Into<TypedValue>) -> Arithmetic;

    /// Creates a bitwise and [`Arithmetic`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// let requirement = memory_ref.bitwise_and(50);
    /// ```
    fn bitwise_and(self, rhs: impl Into<TypedValue>) -> Arithmetic;

    /// Creates a bitwise xor [`Arithmetic`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// let requirement = memory_ref.bitwise_xor(50);
    /// ```
    fn bitwise_xor(self, rhs: impl Into<TypedValue>) -> Arithmetic;
}

impl<T: Into<TypedValue>> TypedValueOps for T {
    fn eq(self, rhs: impl Into<TypedValue>) -> Condition {
        Condition::eq(self, rhs)
    }

    fn ne(self, rhs: impl Into<TypedValue>) -> Condition {
        Condition::ne(self, rhs)
    }

    fn lt(self, rhs: impl Into<TypedValue>) -> Condition {
        Condition::lt(self, rhs)
    }

    fn le(self, rhs: impl Into<TypedValue>) -> Condition {
        Condition::le(self, rhs)
    }

    fn gt(self, rhs: impl Into<TypedValue>) -> Condition {
        Condition::gt(self, rhs)
    }

    fn ge(self, rhs: impl Into<TypedValue>) -> Condition {
        Condition::ge(self, rhs)
    }

    fn add(self, rhs: impl Into<TypedValue>) -> Arithmetic {
        Arithmetic::new(ArithmeticFlag::default(), self, None).add(rhs)
    }

    fn sub(self, rhs: impl Into<TypedValue>) -> Arithmetic {
        Arithmetic::new(ArithmeticFlag::default(), self, None).sub(rhs)
    }

    fn mul(self, rhs: impl Into<TypedValue>) -> Arithmetic {
        Arithmetic::new(ArithmeticFlag::default(), self, None).mul(rhs)
    }

    fn div(self, rhs: impl Into<TypedValue>) -> Arithmetic {
        Arithmetic::new(ArithmeticFlag::default(), self, None).div(rhs)
    }

    fn modulo(self, rhs: impl Into<TypedValue>) -> Arithmetic {
        Arithmetic::new(ArithmeticFlag::default(), self, None).modulo(rhs)
    }

    fn bitwise_and(self, rhs: impl Into<TypedValue>) -> Arithmetic {
        Arithmetic::new(ArithmeticFlag::default(), self, None).bitwise_and(rhs)
    }

    fn bitwise_xor(self, rhs: impl Into<TypedValue>) -> Arithmetic {
        Arithmetic::new(ArithmeticFlag::default(), self, None).bitwise_xor(rhs)
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

impl_arithmetic_flag_traits!(TypedValue, with_arithmetic_flag, Arithmetic);

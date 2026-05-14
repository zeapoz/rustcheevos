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
    // TODO: Implement for T: Into<TypedValue> instead.
    /// Creates a new equals [`ComparisonRequirement`].
    pub fn eq(self, rhs: impl Into<TypedValue>) -> ComparisonRequirement {
        ComparisonRequirement::eq(self, rhs)
    }

    /// Creates a new not equals [`ComparisonRequirement`].
    pub fn ne(self, rhs: impl Into<TypedValue>) -> ComparisonRequirement {
        ComparisonRequirement::ne(self, rhs)
    }

    /// Creates a new less than [`ComparisonRequirement`].
    pub fn lt(self, rhs: impl Into<TypedValue>) -> ComparisonRequirement {
        ComparisonRequirement::lt(self, rhs)
    }

    /// Creates a new less than or equals [`ComparisonRequirement`].
    pub fn le(self, rhs: impl Into<TypedValue>) -> ComparisonRequirement {
        ComparisonRequirement::le(self, rhs)
    }

    /// Creates a new greater than [`ComparisonRequirement`].
    pub fn gt(self, rhs: impl Into<TypedValue>) -> ComparisonRequirement {
        ComparisonRequirement::gt(self, rhs)
    }

    /// Creates a new greater than or equals [`ComparisonRequirement`].
    pub fn ge(self, rhs: impl Into<TypedValue>) -> ComparisonRequirement {
        ComparisonRequirement::ge(self, rhs)
    }

    /// Creates a new add [`ArithmeticRequirement`].
    #[expect(
        clippy::should_implement_trait,
        reason = "not using arithmetic in the traditional sense"
    )]
    pub fn add(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement {
        ArithmeticRequirement::new(crate::types::flag::ArithmeticFlag::default(), self).add(rhs)
    }

    /// Creates a new subtract [`ArithmeticRequirement`].
    #[expect(
        clippy::should_implement_trait,
        reason = "not using arithmetic in the traditional sense"
    )]
    pub fn sub(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement {
        ArithmeticRequirement::new(crate::types::flag::ArithmeticFlag::default(), self).sub(rhs)
    }

    /// Creates a new multiply [`ArithmeticRequirement`].
    #[expect(
        clippy::should_implement_trait,
        reason = "not using arithmetic in the traditional sense"
    )]
    pub fn mul(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement {
        ArithmeticRequirement::new(crate::types::flag::ArithmeticFlag::default(), self).mul(rhs)
    }

    /// Creates a new divide [`ArithmeticRequirement`].
    #[expect(
        clippy::should_implement_trait,
        reason = "not using arithmetic in the traditional sense"
    )]
    pub fn div(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement {
        ArithmeticRequirement::new(crate::types::flag::ArithmeticFlag::default(), self).div(rhs)
    }

    /// Creates a new modulo [`ArithmeticRequirement`].
    pub fn modulo(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement {
        ArithmeticRequirement::new(crate::types::flag::ArithmeticFlag::default(), self).modulo(rhs)
    }

    /// Creates a new bitwise and [`ArithmeticRequirement`].
    pub fn bitwise_and(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement {
        ArithmeticRequirement::new(crate::types::flag::ArithmeticFlag::default(), self)
            .bitwise_and(rhs)
    }

    /// Creates a new bitwise xor [`ArithmeticRequirement`].
    pub fn bitwise_xor(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement {
        ArithmeticRequirement::new(crate::types::flag::ArithmeticFlag::default(), self)
            .bitwise_xor(rhs)
    }

    /// Creates a new arithmetic [`ArithmeticRequirement`].
    #[must_use]
    pub fn with_arithmetic_flag(self, flag: ArithmeticFlag) -> ArithmeticRequirement {
        ArithmeticRequirement::new(flag, self)
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

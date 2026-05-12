use std::{fmt, str::FromStr};

use winnow::Parser;

use crate::{
    ParseError,
    parsers::{parse_value, parse_value_type},
};

use super::{
    memory::MemoryRef,
    requirement::{arithmetic::ArithmeticRequirement, comparison::ComparisonRequirement},
};

/// A value that can be used in a condition.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypedValue {
    Memory(MemoryRef),
    Value(u32),
    Delta(MemoryRef),
    Prior(MemoryRef),
    BCD(MemoryRef),
    Float(f32),
    Invert(MemoryRef),
    Recall,
}

impl TypedValue {
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
    pub fn add(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement {
        ArithmeticRequirement::new(crate::types::flag::ArithmeticFlag::default(), self).add(rhs)
    }

    /// Creates a new subtract [`ArithmeticRequirement`].
    pub fn sub(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement {
        ArithmeticRequirement::new(crate::types::flag::ArithmeticFlag::default(), self).sub(rhs)
    }

    /// Creates a new multiply [`ArithmeticRequirement`].
    pub fn mul(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement {
        ArithmeticRequirement::new(crate::types::flag::ArithmeticFlag::default(), self).mul(rhs)
    }

    /// Creates a new divide [`ArithmeticRequirement`].
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

    /// Converts Memory variant to Delta variant.
    pub fn delta(self) -> Self {
        match self {
            TypedValue::Memory(m) => TypedValue::Delta(m),
            other => other,
        }
    }

    /// Converts Memory variant to Prior variant.
    pub fn prior(self) -> Self {
        match self {
            TypedValue::Memory(m) => TypedValue::Prior(m),
            other => other,
        }
    }

    /// Converts Memory variant to BCD variant.
    pub fn bcd(self) -> Self {
        match self {
            TypedValue::Memory(m) => TypedValue::BCD(m),
            other => other,
        }
    }

    /// Converts Memory variant to Invert variant.
    pub fn invert(self) -> Self {
        match self {
            TypedValue::Memory(m) => TypedValue::Invert(m),
            other => other,
        }
    }

    //// Returns the type of the typed value.
    pub fn value_type(&self) -> ValueType {
        match self {
            TypedValue::Memory(_) => ValueType::Memory,
            TypedValue::Value(_) => ValueType::Value,
            TypedValue::Delta(_) => ValueType::Delta,
            TypedValue::Prior(_) => ValueType::Prior,
            TypedValue::BCD(_) => ValueType::BCD,
            TypedValue::Float(_) => ValueType::Float,
            TypedValue::Invert(_) => ValueType::Invert,
            TypedValue::Recall => ValueType::Recall,
        }
    }
}

impl From<u32> for TypedValue {
    fn from(value: u32) -> Self {
        TypedValue::Value(value)
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
        parse_value
            .parse(s)
            .map_err(|s| ParseError::InvalidValue(s.to_string()))
    }
}

impl fmt::Display for TypedValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = self.value_type().to_string();
        let value = match self {
            TypedValue::Memory(memory) => memory.to_string(),
            TypedValue::Value(value) => value.to_string(),
            TypedValue::Delta(memory) => memory.to_string(),
            TypedValue::Prior(memory) => memory.to_string(),
            TypedValue::BCD(memory) => memory.to_string(),
            TypedValue::Float(value) => value.to_string(),
            TypedValue::Invert(memory) => memory.to_string(),
            TypedValue::Recall => "{recall}".to_string(),
        };
        write!(f, "{prefix}{value}")
    }
}

/// The type of a value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValueType {
    Memory,
    Value,
    Delta,
    Prior,
    BCD,
    Float,
    Invert,
    Recall,
}

impl TryFrom<&str> for ValueType {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "d" => Ok(ValueType::Delta),
            "p" => Ok(ValueType::Prior),
            "b" => Ok(ValueType::BCD),
            "f" => Ok(ValueType::Float),
            "~" => Ok(ValueType::Invert),
            "{recall}" => Ok(ValueType::Recall),
            _ => Err(ParseError::InvalidValue(s.to_string())),
        }
    }
}

impl FromStr for ValueType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_value_type
            .parse(s)
            .map_err(|s| ParseError::InvalidFlag(s.to_string()))
    }
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ValueType::Memory => "",
            ValueType::Value => "",
            ValueType::Delta => "d",
            ValueType::Prior => "p",
            ValueType::BCD => "b",
            ValueType::Float => "f",
            ValueType::Invert => "~",
            ValueType::Recall => "{recall}",
        };
        write!(f, "{s}")
    }
}

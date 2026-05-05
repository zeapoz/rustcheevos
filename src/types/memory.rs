#![allow(clippy::should_implement_trait)]

use super::ParseError;
use std::{fmt, str::FromStr};

macro_rules! memory_size_method {
    ($name:ident, $size:expr) => {
        pub const fn $name(address: usize) -> MemoryRef {
            MemoryRef {
                size: $size,
                address,
                memtype: MemoryType::Standard,
            }
        }
    };
}

macro_rules! condition_method {
    ($name:ident, $op:expr) => {
        pub fn $name<T: Into<MemOrValue>>(self, other: T) -> super::condition::Condition {
            super::condition::Condition {
                source: super::source::Source {
                    reference: self.into(),
                    flag: None,
                    memtype: None,
                },
                op: Some(super::source::Operation {
                    op: $op,
                    target: other.into(),
                }),
                hits: 0,
            }
        }
    };
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BitIndex {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MemorySize {
    BitIndex(BitIndex),
    Lower4,
    Upper4,
    Bits8,
    Bits16,
    Bits24,
    Bits32,
    Bits16BE,
    Bits24BE,
    Bits32BE,
    BitCount,
    Float,
    FloatBE,
    Double32,
    Double32BE,
    MBF32,
    MBF32LE,
}

impl MemorySize {
    pub fn to_prefix(&self) -> &'static str {
        match self {
            MemorySize::Bits8 => "0xH",
            MemorySize::Bits16 => "0x ",
            MemorySize::Bits32 => "0xX",
            MemorySize::Bits24 => "0xW",
            MemorySize::Bits16BE => "0xI",
            MemorySize::Bits24BE => "0xJ",
            MemorySize::Bits32BE => "0xG",
            MemorySize::BitCount => "0xK",
            MemorySize::Lower4 => "0xL",
            MemorySize::Upper4 => "0xU",
            MemorySize::BitIndex(BitIndex::Zero) => "0xM",
            MemorySize::BitIndex(BitIndex::One) => "0xN",
            MemorySize::BitIndex(BitIndex::Two) => "0xO",
            MemorySize::BitIndex(BitIndex::Three) => "0xP",
            MemorySize::BitIndex(BitIndex::Four) => "0xQ",
            MemorySize::BitIndex(BitIndex::Five) => "0xR",
            MemorySize::BitIndex(BitIndex::Six) => "0xS",
            MemorySize::BitIndex(BitIndex::Seven) => "0xT",
            MemorySize::Float => "fF",
            MemorySize::FloatBE => "fB",
            MemorySize::Double32 => "fH",
            MemorySize::Double32BE => "fI",
            MemorySize::MBF32 => "fM",
            MemorySize::MBF32LE => "fL",
        }
    }
}

impl FromStr for MemorySize {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0x " => Ok(MemorySize::Bits16),
            "0xH" => Ok(MemorySize::Bits8),
            "0xX" => Ok(MemorySize::Bits32),
            "0xW" => Ok(MemorySize::Bits24),
            "0xI" => Ok(MemorySize::Bits16BE),
            "0xJ" => Ok(MemorySize::Bits24BE),
            "0xG" => Ok(MemorySize::Bits32BE),
            "0xK" => Ok(MemorySize::BitCount),
            "0xL" => Ok(MemorySize::Lower4),
            "0xU" => Ok(MemorySize::Upper4),
            "0xM" => Ok(MemorySize::BitIndex(BitIndex::Zero)),
            "0xN" => Ok(MemorySize::BitIndex(BitIndex::One)),
            "0xO" => Ok(MemorySize::BitIndex(BitIndex::Two)),
            "0xP" => Ok(MemorySize::BitIndex(BitIndex::Three)),
            "0xQ" => Ok(MemorySize::BitIndex(BitIndex::Four)),
            "0xR" => Ok(MemorySize::BitIndex(BitIndex::Five)),
            "0xS" => Ok(MemorySize::BitIndex(BitIndex::Six)),
            "0xT" => Ok(MemorySize::BitIndex(BitIndex::Seven)),
            "fF" => Ok(MemorySize::Float),
            "fB" => Ok(MemorySize::FloatBE),
            "fH" => Ok(MemorySize::Double32),
            "fI" => Ok(MemorySize::Double32BE),
            "fM" => Ok(MemorySize::MBF32),
            "fL" => Ok(MemorySize::MBF32LE),
            _ => Err(ParseError::UnknownMemorySize(s.to_string())),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum MemoryType {
    #[default]
    Standard,
    Delta,
    Prior,
    BCD,
    Invert,
}

impl MemoryType {
    pub fn to_prefix(&self) -> &'static str {
        match self {
            MemoryType::Standard => "",
            MemoryType::Delta => "d",
            MemoryType::Prior => "p",
            MemoryType::BCD => "b",
            MemoryType::Invert => "~",
        }
    }
}

impl FromStr for MemoryType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "d" => Ok(MemoryType::Delta),
            "p" => Ok(MemoryType::Prior),
            "b" => Ok(MemoryType::BCD),
            "~" => Ok(MemoryType::Invert),
            _ => Err(ParseError::UnknownMemoryType(s.to_string())),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MemoryRef {
    pub size: MemorySize,
    pub address: usize,
    pub memtype: MemoryType,
}

impl MemoryRef {
    pub fn with_memtype(mut self, memtype: MemoryType) -> super::condition::Condition {
        self.memtype = memtype;
        super::condition::Condition {
            source: super::source::Source {
                reference: MemOrValue::Memory(self),
                flag: None,
                memtype: None,
            },
            op: None,
            hits: 0,
        }
    }

    condition_method!(eq, super::operator::Operator::Equals);
    condition_method!(neq, super::operator::Operator::NotEquals);
    condition_method!(gt, super::operator::Operator::GreaterThan);
    condition_method!(gte, super::operator::Operator::GreaterThanOrEquals);
    condition_method!(lt, super::operator::Operator::LessThan);
    condition_method!(lte, super::operator::Operator::LessThanOrEquals);
    condition_method!(add, super::operator::Operator::Add);
    condition_method!(sub, super::operator::Operator::Subtract);
    condition_method!(mul, super::operator::Operator::Multiply);
    condition_method!(div, super::operator::Operator::Divide);

    memory_size_method!(bit0, MemorySize::BitIndex(BitIndex::Zero));
    memory_size_method!(bit1, MemorySize::BitIndex(BitIndex::One));
    memory_size_method!(bit2, MemorySize::BitIndex(BitIndex::Two));
    memory_size_method!(bit3, MemorySize::BitIndex(BitIndex::Three));
    memory_size_method!(bit4, MemorySize::BitIndex(BitIndex::Four));
    memory_size_method!(bit5, MemorySize::BitIndex(BitIndex::Five));
    memory_size_method!(bit6, MemorySize::BitIndex(BitIndex::Six));
    memory_size_method!(bit7, MemorySize::BitIndex(BitIndex::Seven));
    memory_size_method!(lower4, MemorySize::Lower4);
    memory_size_method!(upper4, MemorySize::Upper4);
    memory_size_method!(bits8, MemorySize::Bits8);
    memory_size_method!(bits16, MemorySize::Bits16);
    memory_size_method!(bits24, MemorySize::Bits24);
    memory_size_method!(bits32, MemorySize::Bits32);
    memory_size_method!(bits16be, MemorySize::Bits16BE);
    memory_size_method!(bits24be, MemorySize::Bits24BE);
    memory_size_method!(bits32be, MemorySize::Bits32BE);
    memory_size_method!(bitcount, MemorySize::BitCount);
    memory_size_method!(float, MemorySize::Float);
    memory_size_method!(floatbe, MemorySize::FloatBE);
    memory_size_method!(double, MemorySize::Double32);
    memory_size_method!(doublebe, MemorySize::Double32BE);
    memory_size_method!(mbf, MemorySize::MBF32);
    memory_size_method!(mbfle, MemorySize::MBF32LE);
}

#[derive(Clone, Debug, PartialEq)]
pub enum MemOrValue {
    Memory(MemoryRef),
    Value { value: u32 },
}

impl MemOrValue {
    pub fn value(self) -> Option<u32> {
        match self {
            MemOrValue::Value { value } => Some(value),
            MemOrValue::Memory(_) => None,
        }
    }
}

impl From<u32> for MemOrValue {
    fn from(value: u32) -> Self {
        MemOrValue::Value { value }
    }
}

impl From<MemoryRef> for MemOrValue {
    fn from(memory: MemoryRef) -> Self {
        MemOrValue::Memory(memory)
    }
}

impl From<(MemorySize, usize)> for MemOrValue {
    fn from((size, address): (MemorySize, usize)) -> Self {
        MemOrValue::Memory(MemoryRef {
            size,
            address,
            memtype: MemoryType::Standard,
        })
    }
}

impl fmt::Display for MemOrValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemOrValue::Value { value } => write!(f, "{value}"),
            MemOrValue::Memory(memory) => {
                write!(
                    f,
                    "{}{}{:x}",
                    memory.memtype.to_prefix(),
                    memory.size.to_prefix(),
                    memory.address
                )
            }
        }
    }
}

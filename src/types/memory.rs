use std::{fmt, str::FromStr};
use winnow::Parser;

use crate::{
    impl_arithmetic_flag_traits,
    parsers::ParseError,
    parsers::{parse_memory_ref, parse_memory_size},
    prelude::Measured,
};

use super::{
    flag::ArithmeticFlag,
    requirement::{arithmetic::ArithmeticRequirement, comparison::ComparisonRequirement},
    value::TypedValue,
};

/// A reference to a memory location.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MemoryRef {
    size: MemorySize,
    address: usize,
    access_mode: AccessMode,
}

impl MemoryRef {
    /// Creates a new memory reference.
    ///
    /// # Arguments
    ///
    /// * `size` - The size of the memory reference.
    /// * `address` - The address of the memory reference.
    pub const fn new(size: MemorySize, address: usize) -> Self {
        Self {
            size,
            address,
            access_mode: AccessMode::Memory,
        }
    }

    /// Returns the access mode of the memory reference.
    pub fn size(&self) -> MemorySize {
        self.size
    }

    /// Returns the address of the memory reference.
    pub fn address(&self) -> usize {
        self.address
    }

    /// Returns the access mode of the memory reference.
    pub fn access_mode(&self) -> AccessMode {
        self.access_mode
    }

    /// Sets the access mode of the memory reference.
    pub fn with_access_mode(mut self, access_mode: AccessMode) -> Self {
        self.access_mode = access_mode;
        self
    }

    /// Sets the access mode to [`AccessMode::Memory`].
    pub fn memory(self) -> Self {
        self.with_access_mode(AccessMode::Memory)
    }

    /// Sets the access mode to [`AccessMode::Delta`].
    pub fn delta(self) -> Self {
        self.with_access_mode(AccessMode::Delta)
    }

    /// Sets the access mode to [`AccessMode::Prior`].
    pub fn prior(self) -> Self {
        self.with_access_mode(AccessMode::Prior)
    }

    /// Sets the access mode to [`AccessMode::BCD`].
    pub fn bcd(self) -> Self {
        self.with_access_mode(AccessMode::BCD)
    }

    /// Sets the access mode to [`AccessMode::Invert`].
    pub fn invert(self) -> Self {
        self.with_access_mode(AccessMode::Invert)
    }

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
        ArithmeticRequirement::new(ArithmeticFlag::default(), self).add(rhs)
    }

    /// Creates a new subtract [`ArithmeticRequirement`].
    pub fn sub(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement {
        ArithmeticRequirement::new(ArithmeticFlag::default(), self).sub(rhs)
    }

    /// Creates a new multiply [`ArithmeticRequirement`].
    pub fn mul(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement {
        ArithmeticRequirement::new(ArithmeticFlag::default(), self).mul(rhs)
    }

    /// Creates a new divide [`ArithmeticRequirement`].
    pub fn div(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement {
        ArithmeticRequirement::new(ArithmeticFlag::default(), self).div(rhs)
    }

    /// Creates a new modulo [`ArithmeticRequirement`].
    pub fn modulo(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement {
        ArithmeticRequirement::new(ArithmeticFlag::default(), self).modulo(rhs)
    }

    /// Creates a new bitwise and [`ArithmeticRequirement`].
    pub fn bitwise_and(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement {
        ArithmeticRequirement::new(ArithmeticFlag::default(), self).bitwise_and(rhs)
    }

    /// Creates a new bitwise xor [`ArithmeticRequirement`].
    pub fn bitwise_xor(self, rhs: impl Into<TypedValue>) -> ArithmeticRequirement {
        ArithmeticRequirement::new(ArithmeticFlag::default(), self).bitwise_xor(rhs)
    }

    pub fn with_flag(self, flag: ArithmeticFlag) -> ArithmeticRequirement {
        ArithmeticRequirement::new(flag, self)
    }
}

impl_arithmetic_flag_traits!(MemoryRef, with_flag, ArithmeticRequirement);

impl Measured for MemoryRef {
    type Output = ArithmeticRequirement;

    fn measured(self) -> Self::Output {
        ArithmeticRequirement::new(ArithmeticFlag::Measured, self)
    }
}

impl FromStr for MemoryRef {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_memory_ref
            .parse(s)
            .map_err(|s| ParseError::InvalidMemoryRef(s.to_string()))
    }
}

impl fmt::Display for MemoryRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hex_addr = format!("{:x}", self.address);
        write!(f, "{}{}{hex_addr}", self.access_mode, self.size)
    }
}

/// A bit index.
#[derive(Debug, Clone, Copy, PartialEq)]
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

/// A memory size.
#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub fn parse_bit_size(c: char) -> Result<MemorySize, ParseError> {
        match c {
            'H' => Ok(MemorySize::Bits8),
            ' ' => Ok(MemorySize::Bits16),
            'X' => Ok(MemorySize::Bits32),
            'W' => Ok(MemorySize::Bits24),
            'I' => Ok(MemorySize::Bits16BE),
            'J' => Ok(MemorySize::Bits24BE),
            'G' => Ok(MemorySize::Bits32BE),
            'K' => Ok(MemorySize::BitCount),
            'L' => Ok(MemorySize::Lower4),
            'U' => Ok(MemorySize::Upper4),
            'M' => Ok(MemorySize::BitIndex(BitIndex::Zero)),
            'N' => Ok(MemorySize::BitIndex(BitIndex::One)),
            'O' => Ok(MemorySize::BitIndex(BitIndex::Two)),
            'P' => Ok(MemorySize::BitIndex(BitIndex::Three)),
            'Q' => Ok(MemorySize::BitIndex(BitIndex::Four)),
            'R' => Ok(MemorySize::BitIndex(BitIndex::Five)),
            'S' => Ok(MemorySize::BitIndex(BitIndex::Six)),
            'T' => Ok(MemorySize::BitIndex(BitIndex::Seven)),
            _ => Err(ParseError::InvalidMemorySize(c.to_string())),
        }
    }

    pub fn parse_float_size(c: char) -> Result<MemorySize, ParseError> {
        match c {
            'F' => Ok(MemorySize::Float),
            'B' => Ok(MemorySize::FloatBE),
            'H' => Ok(MemorySize::Double32),
            'I' => Ok(MemorySize::Double32BE),
            'M' => Ok(MemorySize::MBF32),
            'L' => Ok(MemorySize::MBF32LE),
            _ => Err(ParseError::InvalidMemorySize(c.to_string())),
        }
    }
}

impl TryFrom<&str> for MemorySize {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "0xH" => Ok(MemorySize::Bits8),
            "0x " => Ok(MemorySize::Bits16),
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
            _ => Err(ParseError::InvalidMemorySize(s.to_string())),
        }
    }
}

impl FromStr for MemorySize {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_memory_size
            .parse(s)
            .map_err(|s| ParseError::InvalidMemorySize(s.to_string()))
    }
}

impl fmt::Display for MemorySize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
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
            MemorySize::BitIndex(index) => match index {
                BitIndex::Zero => "0xM",
                BitIndex::One => "0xN",
                BitIndex::Two => "0xO",
                BitIndex::Three => "0xP",
                BitIndex::Four => "0xQ",
                BitIndex::Five => "0xR",
                BitIndex::Six => "0xS",
                BitIndex::Seven => "0xT",
            },
            MemorySize::Float => "fF",
            MemorySize::FloatBE => "fB",
            MemorySize::Double32 => "fH",
            MemorySize::Double32BE => "fI",
            MemorySize::MBF32 => "fM",
            MemorySize::MBF32LE => "fL",
        };
        write!(f, "{s}")
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum AccessMode {
    #[default]
    Memory,
    Delta,
    Prior,
    BCD,
    Invert,
}

impl TryFrom<char> for AccessMode {
    type Error = ParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'd' => Ok(AccessMode::Delta),
            'p' => Ok(AccessMode::Prior),
            'b' => Ok(AccessMode::BCD),
            '~' => Ok(AccessMode::Invert),
            _ => Err(ParseError::InvalidMemoryAccessMode(c.to_string())),
        }
    }
}

impl fmt::Display for AccessMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AccessMode::Memory => "",
            AccessMode::Delta => "d",
            AccessMode::Prior => "p",
            AccessMode::BCD => "b",
            AccessMode::Invert => "~",
        };
        write!(f, "{s}")
    }
}

macro_rules! memory_ref_constructors {
    ($($variant:ident($inner:ident::$inner_variant:ident) => $method:ident),*$(,)?) => {
        $(
            impl MemoryRef {
                pub const fn $method(address: usize) -> Self {
                    Self::new(MemorySize::$variant($inner::$inner_variant), address)
                }
            }
        )*
    };
    ($($variant:ident => $method:ident),*$(,)?) => {
        $(
            impl MemoryRef {
                pub const fn $method(address: usize) -> Self {
                    Self::new(MemorySize::$variant, address)
                }
            }
        )*
    };
}

memory_ref_constructors! {
    BitIndex(BitIndex::Zero) => bit0,
    BitIndex(BitIndex::One) => bit1,
    BitIndex(BitIndex::Two) => bit2,
    BitIndex(BitIndex::Three) => bit3,
    BitIndex(BitIndex::Four) => bit4,
    BitIndex(BitIndex::Five) => bit5,
    BitIndex(BitIndex::Six) => bit6,
    BitIndex(BitIndex::Seven) => bit7,
}

memory_ref_constructors! {
    Lower4 => lower4,
    Upper4 => upper4,
    Bits8 => bits8,
    Bits16 => bits16,
    Bits24 => bits24,
    Bits32 => bits32,
    Bits16BE => bits16be,
    Bits24BE => bits24be,
    Bits32BE => bits32be,
    BitCount => bitcount,
    Float => float,
    FloatBE => floatbe,
    Double32 => double32,
    Double32BE => double32be,
    MBF32 => mbf32,
    MBF32LE => mbf32le,
}

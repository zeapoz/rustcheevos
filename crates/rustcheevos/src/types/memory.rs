//! Type definitions for memory references.

use std::{fmt, str::FromStr};
use winnow::Parser;

use crate::{
    impl_arithmetic_flag_traits,
    parsers::ParseError,
    parsers::{parse_memory_ref, parse_memory_size},
    prelude::Measured,
};

use super::{flag::ArithmeticFlag, requirement::arithmetic::Arithmetic};

/// A reference to a memory location.
///
/// This is the core type used for referencing memory locations when building
/// [`Requirement`][`crate::types::requirement::Requirement`]s.
///
/// # Examples
///
/// ```
/// use rustcheevos::prelude::*;
/// use rustcheevos::types::memory::{MemoryRef, MemorySize, AccessMode};
///
/// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234).delta();
/// assert_eq!(memory_ref.size(), MemorySize::Bits8);
/// assert_eq!(memory_ref.address(), 0x1234);
/// assert_eq!(memory_ref.access_mode(), AccessMode::Delta);
/// ```
///
/// The default syntax for constructing a [`MemoryRef`] can be very verbose, so as an alternative,
/// convenience macros are provided for all [`MemorySize`]s. In addition, macros are available for
/// setting [`AccessMode`]s too.
///
/// ```
/// use rustcheevos::prelude::*;
/// use rustcheevos::types::memory::{MemoryRef, MemorySize, AccessMode};
/// use rustcheevos::{bits8, delta};
///
/// let memory_a = MemoryRef::new(MemorySize::Bits8, 0x1234).delta();
/// let memory_b = delta!(bits8!(0x1234));
/// assert_eq!(memory_a, memory_b);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MemoryRef {
    /// The size of the memory reference.
    size: MemorySize,
    /// The address of the memory reference.
    address: usize,
    /// The access mode of the memory reference.
    access_mode: AccessMode,
}

impl MemoryRef {
    /// Creates a new memory reference at the given size and address.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::memory::{MemoryRef, MemorySize, AccessMode};
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// assert_eq!(memory_ref.size(), MemorySize::Bits8);
    /// assert_eq!(memory_ref.address(), 0x1234);
    /// assert_eq!(memory_ref.access_mode(), AccessMode::Memory);
    /// ```
    #[must_use]
    pub const fn new(size: MemorySize, address: usize) -> Self {
        Self {
            size,
            address,
            access_mode: AccessMode::Memory,
        }
    }

    /// Returns the size of the memory reference.
    #[must_use]
    pub fn size(&self) -> MemorySize {
        self.size
    }

    /// Returns the address of the memory reference.
    #[must_use]
    pub fn address(&self) -> usize {
        self.address
    }

    /// Returns the access mode of the memory reference.
    #[must_use]
    pub fn access_mode(&self) -> AccessMode {
        self.access_mode
    }

    /// Sets the access mode of this memory reference.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::memory::{MemoryRef, MemorySize, AccessMode};
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234)
    ///     .with_access_mode(AccessMode::Delta);
    /// assert_eq!(memory_ref.access_mode(), AccessMode::Delta);
    /// ```
    #[must_use]
    pub fn with_access_mode(mut self, access_mode: AccessMode) -> Self {
        self.access_mode = access_mode;
        self
    }

    /// Sets the given flag on this requirement.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::{flag::ArithmeticFlag, memory::{MemoryRef, MemorySize}};
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234);
    /// let arithmetic = memory_ref.with_flag(ArithmeticFlag::AddSource);
    /// assert_eq!(arithmetic.flag(), ArithmeticFlag::AddSource);
    /// ```
    #[must_use]
    pub fn with_flag(self, flag: ArithmeticFlag) -> Arithmetic {
        Arithmetic::new(flag, self, None)
    }
}

impl_arithmetic_flag_traits!(MemoryRef, with_flag, Arithmetic);

impl Measured for MemoryRef {
    type Output = Arithmetic;

    fn measured(self) -> Self::Output {
        Arithmetic::new(ArithmeticFlag::Measured, self, None)
    }
}

impl FromStr for MemoryRef {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_memory_ref
            .parse(s)
            .map_err(|s| ParseError::MemoryRef(s.to_string()))
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
    /// The first bit (index 0).
    Zero,
    /// The second bit (index 1).
    One,
    /// The third bit (index 2).
    Two,
    /// The fourth bit (index 3).
    Three,
    /// The fifth bit (index 4).
    Four,
    /// The sixth bit (index 5).
    Five,
    /// The seventh bit (index 6).
    Six,
    /// The eighth bit (index 7).
    Seven,
}

/// A memory size.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemorySize {
    /// A bit index.
    BitIndex(BitIndex),
    /// The lower 4 bits.
    Lower4,
    /// The upper 4 bits.
    Upper4,
    /// 8 bits.
    Bits8,
    /// 16 bits.
    Bits16,
    /// 24 bits.
    Bits24,
    /// 32 bits.
    Bits32,
    /// 16 bits in big endian.
    Bits16BE,
    /// 24 bits in big endian.
    Bits24BE,
    /// 32 bits in big endian.
    Bits32BE,
    /// The number of bits set to 1.
    BitCount,
    /// A float.
    Float,
    /// A float in big endian.
    FloatBE,
    /// A double.
    Double32,
    /// A double in big endian.
    Double32BE,
    /// An MBF32.
    MBF32,
    /// An MBF32 in little endian.
    MBF32LE,
}

impl MemorySize {
    /// Parses a memory size from a character.
    ///
    /// # Errors
    /// Returns an error if the character is not a valid memory size.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::memory::MemorySize;
    ///
    /// let memory_size = MemorySize::parse_bit_size('H').unwrap();
    /// assert_eq!(memory_size, MemorySize::Bits8);
    /// ```
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
            _ => Err(ParseError::MemorySize(c.to_string())),
        }
    }

    /// Parses a float size from a character.
    ///
    /// # Errors
    /// Returns an error if the character is not a valid float size.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::memory::MemorySize;
    ///
    /// let memory_size = MemorySize::parse_float_size('F').unwrap();
    /// assert_eq!(memory_size, MemorySize::Float);
    /// ```
    pub fn parse_float_size(c: char) -> Result<MemorySize, ParseError> {
        match c {
            'F' => Ok(MemorySize::Float),
            'B' => Ok(MemorySize::FloatBE),
            'H' => Ok(MemorySize::Double32),
            'I' => Ok(MemorySize::Double32BE),
            'M' => Ok(MemorySize::MBF32),
            'L' => Ok(MemorySize::MBF32LE),
            _ => Err(ParseError::MemorySize(c.to_string())),
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
            _ => Err(ParseError::MemorySize(s.to_string())),
        }
    }
}

impl FromStr for MemorySize {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_memory_size
            .parse(s)
            .map_err(|s| ParseError::MemorySize(s.to_string()))
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

/// An access mode defining how or when a memory reference is accessed.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum AccessMode {
    /// The memory reference is accessed as normal.
    #[default]
    Memory,
    /// The memory reference is accessed on the previous frame.
    Delta,
    /// The memory reference is accessed as the previously stored value.
    Prior,
    /// The memory reference is accessed as a binary coded decimal.
    BCD,
    /// The memory reference is accessed by inverting the bits.
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
            _ => Err(ParseError::MemoryAccessMode(c.to_string())),
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

/// A trait for types that can be modified with access modes (delta, prior, bcd, invert).
pub trait AccessModeModifier {
    /// Applies an access mode to this type.
    #[must_use]
    fn with_access_mode(self, access_mode: AccessMode) -> Self;

    /// Sets the access mode to [`AccessMode::Memory`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::memory::{MemoryRef, MemorySize, AccessMode};
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234).memory();
    /// assert_eq!(memory_ref.access_mode(), AccessMode::Memory);
    /// ```
    #[must_use]
    fn memory(self) -> Self
    where
        Self: Sized,
    {
        self.with_access_mode(AccessMode::Memory)
    }

    /// Sets the access mode to [`AccessMode::Delta`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::memory::{MemoryRef, MemorySize, AccessMode};
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234).delta();
    /// assert_eq!(memory_ref.access_mode(), AccessMode::Delta);
    /// ```
    #[must_use]
    fn delta(self) -> Self
    where
        Self: Sized,
    {
        self.with_access_mode(AccessMode::Delta)
    }

    /// Sets the access mode to [`AccessMode::Prior`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::memory::{MemoryRef, MemorySize, AccessMode};
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234).prior();
    /// assert_eq!(memory_ref.access_mode(), AccessMode::Prior);
    /// ```
    #[must_use]
    fn prior(self) -> Self
    where
        Self: Sized,
    {
        self.with_access_mode(AccessMode::Prior)
    }

    /// Sets the access mode to [`AccessMode::BCD`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::memory::{MemoryRef, MemorySize, AccessMode};
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234).bcd();
    /// assert_eq!(memory_ref.access_mode(), AccessMode::BCD);
    /// ```
    #[must_use]
    fn bcd(self) -> Self
    where
        Self: Sized,
    {
        self.with_access_mode(AccessMode::BCD)
    }

    /// Sets the access mode to [`AccessMode::Invert`].
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::memory::{MemoryRef, MemorySize, AccessMode};
    ///
    /// let memory_ref = MemoryRef::new(MemorySize::Bits8, 0x1234).invert();
    /// assert_eq!(memory_ref.access_mode(), AccessMode::Invert);
    /// ```
    #[must_use]
    fn invert(self) -> Self
    where
        Self: Sized,
    {
        self.with_access_mode(AccessMode::Invert)
    }
}

impl AccessModeModifier for MemoryRef {
    fn with_access_mode(mut self, access_mode: AccessMode) -> Self {
        self.access_mode = access_mode;
        self
    }
}

#[allow(missing_docs, clippy::missing_docs_in_private_items)]
macro_rules! memory_ref_constructors {
    ($($variant:ident($inner:ident::$inner_variant:ident) => $method:ident),*$(,)?) => {
        impl MemoryRef {
            $(
                    #[allow(missing_docs, clippy::missing_docs_in_private_items)]
                    #[must_use] pub const fn $method(address: usize) -> Self {
                        Self::new(MemorySize::$variant($inner::$inner_variant), address)
                    }
            )*
        }
    };
    ($($variant:ident => $method:ident),*$(,)?) => {
        impl MemoryRef {
            $(
                    #[allow(missing_docs, clippy::missing_docs_in_private_items)]
                    #[must_use] pub const fn $method(address: usize) -> Self {
                        Self::new(MemorySize::$variant, address)
                    }
            )*
        }
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

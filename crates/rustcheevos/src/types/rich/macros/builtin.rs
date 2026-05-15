//! Builtin rich presence macros.

use std::fmt;

use crate::types::rich::format::FormatType;

/// Builtin macros for rich presence display formatting.
#[derive(Debug, Clone, PartialEq)]
pub enum BuiltInMacro {
    /// Integer format.
    Number,
    /// Unsigned integer format.
    Unsigned,
    /// Score format.
    Score,
    /// Centiseconds format (displays as milliseconds / 10).
    Centiseconds,
    /// Seconds format.
    Seconds,
    /// Minutes format.
    Minutes,
    /// Fixed-point with 1 decimal place.
    Fixed1,
    /// Fixed-point with 2 decimal places.
    Fixed2,
    /// Fixed-point with 3 decimal places.
    Fixed3,
    /// Float with 1 decimal place.
    Float1,
    /// Float with 2 decimal places.
    Float2,
    /// Float with 3 decimal places.
    Float3,
    /// Float with 4 decimal places.
    Float4,
    /// Float with 5 decimal places.
    Float5,
    /// Float with 6 decimal places.
    Float6,
    /// ASCII character format.
    ASCIIChar,
    /// Unicode character format.
    UnicodeChar,
}

impl BuiltInMacro {
    /// Returns the corresponding [`FormatType`] for this macro, if it exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::types::rich::macros::builtin::BuiltInMacro;
    ///
    /// let format = BuiltInMacro::Score.format_type();
    /// assert!(format.is_some());
    /// ```
    #[must_use]
    pub fn format_type(&self) -> Option<FormatType> {
        match self {
            Self::Number | Self::Unsigned => Some(FormatType::Unsigned),
            Self::Score => Some(FormatType::Score),
            Self::Centiseconds => Some(FormatType::Milliseconds),
            Self::Seconds => Some(FormatType::Seconds),
            Self::Minutes => Some(FormatType::Minutes),
            Self::Fixed1 => Some(FormatType::Fixed1),
            Self::Fixed2 => Some(FormatType::Fixed2),
            Self::Fixed3 => Some(FormatType::Fixed3),
            Self::Float1 => Some(FormatType::Float1),
            Self::Float2 => Some(FormatType::Float2),
            Self::Float3 => Some(FormatType::Float3),
            Self::Float4 => Some(FormatType::Float4),
            Self::Float5 => Some(FormatType::Float5),
            Self::Float6 => Some(FormatType::Float6),
            _ => None,
        }
    }
}

impl fmt::Display for BuiltInMacro {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Number => "Number",
            Self::Unsigned => "Unsigned",
            Self::Score => "Score",
            Self::Centiseconds => "Centiseconds",
            Self::Seconds => "Seconds",
            Self::Minutes => "Minutes",
            Self::Fixed1 => "Fixed1",
            Self::Fixed2 => "Fixed2",
            Self::Fixed3 => "Fixed3",
            Self::Float1 => "Float1",
            Self::Float2 => "Float2",
            Self::Float3 => "Float3",
            Self::Float4 => "Float4",
            Self::Float5 => "Float5",
            Self::Float6 => "Float6",
            Self::ASCIIChar => "ASCIIChar",
            Self::UnicodeChar => "UnicodeChar",
        };
        write!(f, "{s}")
    }
}

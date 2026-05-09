use std::fmt;

use crate::types::rich::format::FormatType;

/// Built in macros.
#[derive(Debug, Clone, PartialEq)]
pub enum BuiltInMacro {
    Number,
    Unsigned,
    Score,
    Centiseconds,
    Seconds,
    Minutes,
    Fixed1,
    Fixed2,
    Fixed3,
    Float1,
    Float2,
    Float3,
    Float4,
    Float5,
    Float6,
    ASCIIChar,
    UnicodeChar,
}

impl BuiltInMacro {
    /// Returns the corresponding [`FormatType`] for this macro, if it exists.
    pub fn format_type(&self) -> Option<FormatType> {
        match self {
            Self::Number => Some(FormatType::Unsigned),
            Self::Unsigned => Some(FormatType::Unsigned),
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

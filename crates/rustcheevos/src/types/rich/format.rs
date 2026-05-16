//! Type definition for rich presence formats.

use std::fmt;

/// A rich presence format definition.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Format {
    /// The name of the format.
    name: String,
    /// The format type.
    format_type: FormatType,
}

impl Format {
    /// Creates a new rich presence format definition.
    pub fn new(name: impl Into<String>, format_type: FormatType) -> Self {
        Self {
            name: name.into(),
            format_type,
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Format:{}", self.name)?;
        writeln!(f, "FormatType={}", self.format_type)
    }
}

/// Rich presence format types.
///
/// # Examples
///
/// ```
/// use rustcheevos::prelude::FormatType;
///
/// let format_type = FormatType::Seconds;
/// ```
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum FormatType {
    /// A score.
    Score,
    /// A frame counnt.
    Frames,
    /// Milliseconds.
    Milliseconds,
    /// Seconds.
    Seconds,
    /// Minutes.
    Minutes,
    /// Seconds as minutes.
    SecsAsMins,
    /// A value.
    #[default]
    Value,
    /// An unsigned value.
    Unsigned,
    /// Tens.
    Tens,
    /// Hundreds.
    Hundreds,
    /// Thousands.
    Thousands,
    /// Fixed 1.
    Fixed1,
    /// Fixed 2.
    Fixed2,
    /// Fixed 3.
    Fixed3,
    /// Points.
    Points,
    /// Float 1.
    Float1,
    /// Float 2.
    Float2,
    /// Float 3.
    Float3,
    /// Float 4.
    Float4,
    /// Float 5.
    Float5,
    /// Float 6.
    Float6,
}

impl fmt::Display for FormatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            FormatType::Score => "SCORE",
            FormatType::Frames => "FRAMES",
            FormatType::Milliseconds => "MILLISECONDS",
            FormatType::Seconds => "SECONDS",
            FormatType::Minutes => "MINUTES",
            FormatType::SecsAsMins => "SECS_AS_MINS",
            FormatType::Value => "VALUE",
            FormatType::Unsigned => "UNSIGNED",
            FormatType::Tens => "TENS",
            FormatType::Hundreds => "HUNDREDS",
            FormatType::Thousands => "THOUSANDS",
            FormatType::Fixed1 => "FIXED1",
            FormatType::Fixed2 => "FIXED2",
            FormatType::Fixed3 => "FIXED3",
            FormatType::Points => "POINTS",
            FormatType::Float1 => "FLOAT1",
            FormatType::Float2 => "FLOAT2",
            FormatType::Float3 => "FLOAT3",
            FormatType::Float4 => "FLOAT4",
            FormatType::Float5 => "FLOAT5",
            FormatType::Float6 => "FLOAT6",
        };
        write!(f, "{s}")
    }
}

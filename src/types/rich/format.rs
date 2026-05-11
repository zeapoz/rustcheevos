use std::fmt;

/// A rich presence format definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Format {
    pub name: String,
    pub format_type: FormatType,
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
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum FormatType {
    Score,
    Frames,
    Milliseconds,
    Seconds,
    Minutes,
    SecsAsMins,
    #[default]
    Value,
    Unsigned,
    Tens,
    Hundreds,
    Thousands,
    Fixed1,
    Fixed2,
    Fixed3,
    Points,
    Float1,
    Float2,
    Float3,
    Float4,
    Float5,
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

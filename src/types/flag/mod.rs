use std::{fmt, str::FromStr};

use winnow::Parser;

use crate::{parsers::ParseError, parsers::parse_flag};

pub mod traits;

/// A flag that modifies behavior.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Flag {
    /// A flag used in comparisons.
    Comparison(ComparisonFlag),
    /// A flag in an arithmetic expression.
    Arithmetic(ArithmeticFlag),
}

impl From<ComparisonFlag> for Flag {
    fn from(flag: ComparisonFlag) -> Self {
        Flag::Comparison(flag)
    }
}

impl From<ArithmeticFlag> for Flag {
    fn from(flag: ArithmeticFlag) -> Self {
        Flag::Arithmetic(flag)
    }
}

impl FromStr for Flag {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_flag
            .parse(s)
            .map_err(|s| ParseError::InvalidFlag(s.to_string()))
    }
}

impl fmt::Display for Flag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Flag::Comparison(flag) => write!(f, "{flag}"),
            Flag::Arithmetic(flag) => write!(f, "{flag}"),
        }
    }
}

/// A flag used in comparisons.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ComparisonFlag {
    PauseIf,
    ResetIf,
    ResetNextIf,
    AddHits,
    SubHits,
    AndNext,
    OrNext,
    Measured,
    MeasuredPercentage,
    MeasuredIf,
    Trigger,
}

impl TryFrom<char> for ComparisonFlag {
    type Error = ParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'P' => Ok(ComparisonFlag::PauseIf),
            'R' => Ok(ComparisonFlag::ResetIf),
            'Z' => Ok(ComparisonFlag::ResetNextIf),
            'C' => Ok(ComparisonFlag::AddHits),
            'D' => Ok(ComparisonFlag::SubHits),
            'N' => Ok(ComparisonFlag::AndNext),
            'O' => Ok(ComparisonFlag::OrNext),
            'M' => Ok(ComparisonFlag::Measured),
            'G' => Ok(ComparisonFlag::MeasuredPercentage),
            'Q' => Ok(ComparisonFlag::MeasuredIf),
            'T' => Ok(ComparisonFlag::Trigger),
            _ => Err(ParseError::InvalidFlag(c.to_string())),
        }
    }
}

impl fmt::Display for ComparisonFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ComparisonFlag::PauseIf => "P",
            ComparisonFlag::ResetIf => "R",
            ComparisonFlag::ResetNextIf => "Z",
            ComparisonFlag::AddHits => "C",
            ComparisonFlag::SubHits => "D",
            ComparisonFlag::AndNext => "N",
            ComparisonFlag::OrNext => "O",
            ComparisonFlag::Measured => "M",
            ComparisonFlag::MeasuredPercentage => "G",
            ComparisonFlag::MeasuredIf => "Q",
            ComparisonFlag::Trigger => "T",
        };
        write!(f, "{s}:")
    }
}

/// A flag in an arithmetic expression.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum ArithmeticFlag {
    #[default]
    AddSource,
    SubSource,
    AddAddress,
    Remember,
    // A special exception only to be used in a leaderboard value group.
    Measured,
}

impl TryFrom<char> for ArithmeticFlag {
    type Error = ParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(ArithmeticFlag::AddSource),
            'B' => Ok(ArithmeticFlag::SubSource),
            'I' => Ok(ArithmeticFlag::AddAddress),
            'K' => Ok(ArithmeticFlag::Remember),
            'M' => Ok(ArithmeticFlag::Measured),
            _ => Err(ParseError::InvalidFlag(c.to_string())),
        }
    }
}

impl fmt::Display for ArithmeticFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ArithmeticFlag::AddSource => "A",
            ArithmeticFlag::SubSource => "B",
            ArithmeticFlag::AddAddress => "I",
            ArithmeticFlag::Remember => "K",
            ArithmeticFlag::Measured => "M",
        };
        write!(f, "{s}:")
    }
}

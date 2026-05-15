//! Type definitions for flags.

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
            .map_err(|s| ParseError::Flag(s.to_string()))
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
    /// Pause if the condition is met.
    PauseIf,
    /// Reset if the condition is met.
    ResetIf,
    /// Reset next if the condition is met.
    ResetNextIf,
    /// Add hits to the condition.
    AddHits,
    /// Subtract hits from the condition.
    SubHits,
    /// And next with the following condition.
    AndNext,
    /// Or next with the following condition.
    OrNext,
    /// Measure the condition.
    Measured,
    /// Measure the condition as a percentage.
    MeasuredPercentage,
    /// Measure the condition if the condition is met.
    MeasuredIf,
    /// Trigger on the condition.
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
            _ => Err(ParseError::Flag(c.to_string())),
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

/// A flag used in an arithmetic expression.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum ArithmeticFlag {
    /// Add a source to the accumulator.
    #[default]
    AddSource,
    /// Sub a source from the accumulator.
    SubSource,
    /// Add a address to the accumulator.
    AddAddress,
    /// Remember a value.
    Remember,
    // :HACK: A special exception only to be used in a leaderboard value group.
    /// Measure a value.
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
            _ => Err(ParseError::Flag(c.to_string())),
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

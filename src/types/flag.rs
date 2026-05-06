//! Condition flags for achievement conditions.

use crate::types::ParseError;
use crate::types::condition::Condition;
use std::str::FromStr;

/// Flags that modify condition behavior.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Flag {
    /// Pause the achievement when this condition is true.
    PauseIf,
    /// Reset the achievement when this condition is true.
    ResetIf,
    /// Reset the next condition when this condition is true.
    ResetNextIf,
    /// Add to the source value.
    AddSource,
    /// Subtract from the source value.
    SubSource,
    /// Add to the hits counter.
    AddHits,
    /// Subtract from the hits counter.
    SubHits,
    /// Add to the address.
    AddAddress,
    /// AND this condition with the next one.
    AndNext,
    /// OR this condition with the next one.
    OrNext,
    /// Use this value for measured progress.
    Measured,
    /// Use this value as a percentage for measured progress.
    MeasuredPercentage,
    /// Use this value for measured progress if true.
    MeasuredIf,
    /// This is a trigger condition.
    Trigger,
    /// Remember this value for later.
    Remember,
}

impl Flag {
    /// Returns the string prefix for this flag.
    ///
    /// # Returns
    ///
    /// The string prefix.
    pub fn to_prefix(&self) -> &'static str {
        match self {
            Flag::PauseIf => "P:",
            Flag::ResetIf => "R:",
            Flag::ResetNextIf => "Z:",
            Flag::AddSource => "A:",
            Flag::SubSource => "B:",
            Flag::AddHits => "C:",
            Flag::SubHits => "D:",
            Flag::AddAddress => "I:",
            Flag::AndNext => "N:",
            Flag::OrNext => "O:",
            Flag::Measured => "M:",
            Flag::MeasuredPercentage => "G:",
            Flag::MeasuredIf => "Q:",
            Flag::Trigger => "T:",
            Flag::Remember => "K:",
        }
    }
}

impl FromStr for Flag {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "P:" => Ok(Flag::PauseIf),
            "R:" => Ok(Flag::ResetIf),
            "Z:" => Ok(Flag::ResetNextIf),
            "A:" => Ok(Flag::AddSource),
            "B:" => Ok(Flag::SubSource),
            "C:" => Ok(Flag::AddHits),
            "D:" => Ok(Flag::SubHits),
            "I:" => Ok(Flag::AddAddress),
            "N:" => Ok(Flag::AndNext),
            "O:" => Ok(Flag::OrNext),
            "M:" => Ok(Flag::Measured),
            "G:" => Ok(Flag::MeasuredPercentage),
            "Q:" => Ok(Flag::MeasuredIf),
            "T:" => Ok(Flag::Trigger),
            "K:" => Ok(Flag::Remember),
            _ => Err(ParseError::UnknownFlag(s.to_string())),
        }
    }
}

/// Extension trait for adding flags to conditions.
pub trait WithFlagExt {
    /// The output type after adding the flag.
    type Output;
    /// Adds a flag to the condition(s).
    fn with_flag(self, flag: Flag) -> Self::Output;
}

impl<const N: usize> WithFlagExt for [Condition; N] {
    type Output = [Condition; N];
    fn with_flag(mut self, flag: Flag) -> Self::Output {
        for item in self.iter_mut() {
            item.source.flag = Some(flag);
        }
        self
    }
}

impl WithFlagExt for Vec<Condition> {
    type Output = Vec<Condition>;
    fn with_flag(mut self, flag: Flag) -> Self::Output {
        for cond in self.iter_mut() {
            cond.source.flag = Some(flag);
        }
        self
    }
}

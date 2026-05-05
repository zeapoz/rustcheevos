use crate::types::ParseError;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Flag {
    PauseIf,
    ResetIf,
    ResetNextIf,
    AddSource,
    SubSource,
    AddHits,
    SubHits,
    AddAddress,
    AndNext,
    OrNext,
    Measured,
    MeasuredPercentage,
    MeasuredIf,
    Trigger,
    Remember,
}

impl Flag {
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

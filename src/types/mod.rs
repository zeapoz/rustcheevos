//! Type definitions for memory addresses, conditions, achievements, and leaderboards.

use thiserror::Error;

/// Common types for memory parsing.
pub(crate) mod accessors;
pub mod achievement;
pub mod condition;
pub mod flag;
pub mod leaderboard;
pub mod memory;
pub mod operator;
pub mod set;
pub mod source;

/// Errors that can occur when parsing memory addresses and achievement definitions.
#[derive(Error, Debug)]
pub enum ParseError {
    /// Unknown memory size prefix.
    #[error("unknown memory size prefix: {0}")]
    UnknownMemorySize(String),

    /// Unknown memory type prefix.
    #[error("unknown memory type prefix: {0}")]
    UnknownMemoryType(String),

    /// Unknown flag.
    #[error("unknown flag: {0}")]
    UnknownFlag(String),

    /// Unknown operator.
    #[error("unknown operator: {0}")]
    UnknownOperator(String),

    /// Invalid value.
    #[error("invalid value")]
    InvalidValue,

    /// Empty input.
    #[error("empty input")]
    EmptyInput,

    /// Invalid format.
    #[error("invalid format")]
    InvalidFormat,

    /// Invalid entry type.
    #[error("invalid entry type: {0}")]
    InvalidEntryType(String),

    /// Invalid leaderboard.
    #[error("invalid leaderboard: {0}")]
    InvalidLeaderboard(String),

    /// Invalid note.
    #[error("invalid note: {0}")]
    InvalidNote(String),

    /// Invalid tag.
    #[error("invalid tag: {0}")]
    InvalidTag(String),

    /// Operator requires a target value.
    #[error("operator requires target value")]
    OperatorRequiresTarget,
}

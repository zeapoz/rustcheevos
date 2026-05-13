//! Parser functions for various types.

use thiserror::Error;

pub use flag::*;
pub use memory::*;
pub use operator::*;
pub use requirement::*;
pub use value::*;

mod flag;
mod memory;
mod operator;
mod requirement;
mod value;

/// The error type for parsing errors.
#[derive(Error, Debug, Clone)]
pub enum ParseError {
    /// Invalid flag.
    #[error("invalid flag: {0}")]
    Flag(String),

    /// Invalid operator.
    #[error("invalid operator: {0}")]
    Operator(String),

    /// Invalid memory access mode.
    #[error("invalid memory access mode: {0}")]
    MemoryAccessMode(String),

    /// Invalid memory size.
    #[error("invalid memory size: {0}")]
    MemorySize(String),

    /// Invalid memory ref.
    #[error("invalid memory ref: {0}")]
    MemoryRef(String),

    /// Invalid value.
    #[error("invalid value: {0}")]
    Value(String),

    /// Invalid hit count.
    #[error("invalid hit count: {0}")]
    HitCount(String),

    /// Invalid requirement.
    #[error("invalid requirement: {0}")]
    Requirement(String),

    /// Invalid achievement tag.
    #[error("invalid tag: {0}")]
    Tag(String),

    /// Invalid leaderboard format.
    #[error("invalid leaderboard: {0}")]
    Leaderboard(String),
}

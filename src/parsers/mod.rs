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

#[derive(Error, Debug, Clone)]
pub enum ParseError {
    #[error("invalid flag: {0}")]
    Flag(String),

    #[error("invalid operator: {0}")]
    Operator(String),

    #[error("invalid memory access mode: {0}")]
    MemoryAccessMode(String),

    #[error("invalid memory size: {0}")]
    MemorySize(String),

    #[error("invalid memory ref: {0}")]
    MemoryRef(String),

    #[error("invalid value: {0}")]
    Value(String),

    #[error("invalid hit count: {0}")]
    HitCount(String),

    #[error("invalid requirement: {0}")]
    Requirement(String),

    #[error("invalid tag: {0}")]
    Tag(String),

    #[error("invalid leaderboard: {0}")]
    Leaderboard(String),
}

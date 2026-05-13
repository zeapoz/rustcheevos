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
    InvalidFlag(String),

    #[error("invalid operator: {0}")]
    InvalidOperator(String),

    #[error("invalid memory access mode: {0}")]
    InvalidMemoryAccessMode(String),

    #[error("invalid memory size: {0}")]
    InvalidMemorySize(String),

    #[error("invalid memory ref: {0}")]
    InvalidMemoryRef(String),

    #[error("invalid value: {0}")]
    InvalidValue(String),

    #[error("invalid hit count: {0}")]
    InvalidHitCount(String),

    #[error("invalid requirement: {0}")]
    InvalidRequirement(String),

    #[error("invalid tag: {0}")]
    InvalidTag(String),

    #[error("invalid leaderboard: {0}")]
    InvalidLeaderboardFormat(String),
}

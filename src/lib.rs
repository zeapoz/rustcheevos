use thiserror::Error;

mod macros;
pub(crate) mod parsers;
pub mod prelude;
pub mod schema;
pub mod types;

#[derive(Error, Debug, Clone)]
pub enum ParseError {
    #[error("invalid flag: {0}")]
    InvalidFlag(String),

    #[error("invalid operator: {0}")]
    InvalidOperator(String),

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

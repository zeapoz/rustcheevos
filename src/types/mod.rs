use thiserror::Error;

pub mod accessors;
pub mod achievement;
pub mod condition;
pub mod flag;
pub mod memory;
pub mod operator;
pub mod set;
pub mod source;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("unknown memory size prefix: {0}")]
    UnknownMemorySize(String),

    #[error("unknown memory type prefix: {0}")]
    UnknownMemoryType(String),

    #[error("unknown flag: {0}")]
    UnknownFlag(String),

    #[error("unknown operator: {0}")]
    UnknownOperator(String),

    #[error("invalid value")]
    InvalidValue,

    #[error("empty input")]
    EmptyInput,

    #[error("invalid format")]
    InvalidFormat,

    #[error("invalid entry type: {0}")]
    InvalidEntryType(String),

    #[error("invalid leaderboard: {0}")]
    InvalidLeaderboard(String),

    #[error("invalid note: {0}")]
    InvalidNote(String),

    #[error("invalid tag: {0}")]
    InvalidTag(String),

    #[error("operator requires target value")]
    OperatorRequiresTarget,
}

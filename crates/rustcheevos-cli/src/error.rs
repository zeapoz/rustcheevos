use std::{fmt, io};

/// Error type for CLI operations.
#[derive(Debug, thiserror::Error)]
pub enum CliError {
    /// An I/O error occurred.
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// A formatting error occurred.
    #[error("format error: {0}")]
    Fmt(#[from] fmt::Error),
}

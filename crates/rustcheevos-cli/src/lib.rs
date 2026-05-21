//! Game CLI helpers for Rustcheevos projects.
//!
//! Provides a CLI interface for projects that use Rustcheevos.
//!
//! # Example
//!
//! ```no_run
//! use rustcheevos::types::game::GameData;
//! use rustcheevos_cli::RustcheevosCli;
//!
//! fn main() -> Result<(), rustcheevos_cli::CliError> {
//!     let mut game_data = GameData::new(1234, "My Game");
//!     // ... add assets ...
//!     RustcheevosCli::parse().run(&game_data)
//! }
//! ```

use clap::{Parser, Subcommand};
use std::path::PathBuf;

use rustcheevos::types::game::GameData;

use crate::export::export;
use crate::readme::generate_readme;

/// Error types for CLI operations.
mod error;
mod export;
mod readme;

pub use error::CliError;

/// Default output directory for the export command.
const DEFAULT_OUTPUT_DIR: &str = "output";
/// Default output path for the readme command.
const DEFAULT_README_PATH: &str = "README.md";
/// Default author for achievement entries.
const DEFAULT_AUTHOR: &str = "Rustcheevos";

/// Verbosity level for CLI output.
#[derive(Debug, Clone, Copy, Default)]
enum Verbosity {
    /// Suppress all output except errors.
    Quiet,
    /// Show summary output.
    #[default]
    Normal,
    /// Show detailed output.
    Verbose,
}

impl Verbosity {
    /// Resolves verbosity from mutually exclusive flags.
    fn from_flags(quiet: bool, verbose: bool) -> Self {
        if quiet {
            Self::Quiet
        } else if verbose {
            Self::Verbose
        } else {
            Self::Normal
        }
    }
}

/// Embeddable command-line interface for Rustcheevos projects.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct RustcheevosCli {
    /// The subcommand to execute.
    #[command(subcommand)]
    command: RustcheevosCommand,
}

/// Available subcommands.
#[derive(Debug, Subcommand)]
enum RustcheevosCommand {
    /// Export game assets to disk.
    Export {
        /// Output directory for exported files.
        #[arg(long, short, default_value = DEFAULT_OUTPUT_DIR)]
        output: PathBuf,
        /// Achievement author for exported files.
        #[arg(long, short, default_value = DEFAULT_AUTHOR)]
        author: String,
        /// Suppress all output except errors.
        #[arg(long, short, group = "verbosity")]
        quiet: bool,
        /// Show detailed output.
        #[arg(long, short, group = "verbosity")]
        verbose: bool,
    },
    /// Generate a README file for the game.
    Readme {
        /// Output path for the generated README.
        #[arg(long, short, default_value = DEFAULT_README_PATH)]
        output: PathBuf,
        /// Path to a file containing supported hashes (format: hash, name per line).
        #[arg(long)]
        hashes: Option<PathBuf>,
    },
}

impl RustcheevosCli {
    /// Parse CLI arguments from `std::env::args`.
    #[must_use]
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }

    /// Run the CLI command with the given game data.
    ///
    /// # Errors
    /// Returns an error if the command fails.
    pub fn run(self, game_data: &GameData) -> Result<(), CliError> {
        match self.command {
            RustcheevosCommand::Export {
                output,
                author,
                quiet,
                verbose,
            } => export(
                game_data,
                &output,
                author,
                Verbosity::from_flags(quiet, verbose),
            ),
            RustcheevosCommand::Readme { output, hashes } => {
                generate_readme(game_data, &output, hashes.as_deref())
            }
        }
    }
}

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

use rustcheevos::types::game::GameData;

use crate::export::{ExportArgs, export};
use crate::readme::{ReadmeArgs, generate_readme};

mod error;
mod export;
mod readme;

pub use error::CliError;

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
    Export(ExportArgs),
    /// Generate a README file for the game.
    Readme(ReadmeArgs),
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
            RustcheevosCommand::Export(args) => export(game_data, args),
            RustcheevosCommand::Readme(args) => generate_readme(game_data, args),
        }
    }
}

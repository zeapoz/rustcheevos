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

#[cfg(feature = "export")]
mod export;
#[cfg(feature = "preview")]
mod preview;
#[cfg(feature = "readme")]
mod readme;

mod error;

pub use error::CliError;

/// Verbosity level for CLI output.
#[cfg(feature = "export")]
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

#[cfg(feature = "export")]
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
    #[cfg(feature = "export")]
    Export(export::Args),
    /// Generate a README file for the game.
    #[cfg(feature = "readme")]
    Readme(readme::Args),
    /// Preview the output of a given asset.
    #[cfg(feature = "preview")]
    Preview(preview::Args),
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
            #[cfg(feature = "export")]
            RustcheevosCommand::Export(args) => export::export(game_data, args),
            #[cfg(feature = "readme")]
            RustcheevosCommand::Readme(args) => readme::generate_readme(game_data, args),
            #[cfg(feature = "preview")]
            RustcheevosCommand::Preview(args) => {
                preview::preview_output(game_data, &args);
                Ok(())
            }
        }
    }
}

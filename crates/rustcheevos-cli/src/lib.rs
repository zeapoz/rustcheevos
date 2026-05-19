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

use clap::Parser;
use std::path::PathBuf;

use rustcheevos::types::game::GameData;

use crate::export::export;
use crate::readme::generate_readme;

/// Error types for CLI operations.
mod error;
mod export;
mod readme;

pub use error::CliError;

/// Embeddable command-line interface for Rustcheevos projects.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct RustcheevosCli {
    /// The subcommand to execute.
    #[command(subcommand)]
    command: GameCommand,
}

/// Available subcommands.
#[derive(Debug, clap::Subcommand)]
pub enum GameCommand {
    /// Export game assets to disk.
    Export {
        /// Output directory for exported files.
        #[arg(long, short, default_value = "output")]
        output: PathBuf,
    },
    /// Generate a README file for the game.
    Readme {
        /// Output path for the generated README.
        #[arg(long, short, default_value = "README.md")]
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
            GameCommand::Export { output } => export(game_data, &output).map_err(CliError::from),
            GameCommand::Readme { output, hashes } => {
                generate_readme(game_data, &output, hashes.as_deref())
            }
        }
    }
}

//! Export game assets to disk.

mod report;

use std::fs;
use std::path::{Path, PathBuf};

use clap::Parser;
use rustcheevos::types::game::GameData;
use rustcheevos_schema::rich::{RICH_PRESENCE_FILE_EXTENSION, RICH_PRESENCE_FILE_SUFFIX};
use rustcheevos_schema::user::{USER_FILE_EXTENSION, USER_FILE_SUFFIX};

use crate::CliError;
use crate::Verbosity;

use self::report::ExportReport;

/// Default output directory for the export command.
const DEFAULT_OUTPUT_DIR: &str = "output";
/// Default author for achievement entries.
const DEFAULT_AUTHOR: &str = "Rustcheevos";

/// Arguments used in the export command.
#[derive(Parser, Debug, Default, Clone)]
pub struct ExportArgs {
    /// Output directory for exported files.
    #[arg(long, short, default_value = DEFAULT_OUTPUT_DIR)]
    pub output: PathBuf,
    /// Achievement author for exported files.
    #[arg(long, short, default_value = DEFAULT_AUTHOR)]
    pub author: String,
    /// Suppress all output except errors.
    #[arg(long, short, group = "verbosity")]
    pub quiet: bool,
    /// Show detailed output.
    #[arg(long, short, group = "verbosity")]
    pub verbose: bool,
    /// Show what would be exported without writing files.
    #[arg(long)]
    pub dry_run: bool,
}

/// Export game assets to disk.
///
/// # Errors
/// Returns an error if the export fails.
pub fn export(
    game_data: &GameData,
    ExportArgs {
        output,
        author,
        dry_run,
        quiet,
        verbose,
    }: ExportArgs,
) -> Result<(), CliError> {
    let verbosity = Verbosity::from_flags(quiet, verbose);

    if !dry_run {
        fs::create_dir_all(&output)?;
    }

    let mut report = ExportReport::new(dry_run);

    let has_user_file = !game_data.achievements().is_empty()
        || !game_data.leaderboards().is_empty()
        || !game_data.code_notes().is_empty();
    if has_user_file {
        let path = user_file_path(game_data, &output);
        if !dry_run {
            let user_file = game_data.to_user_file(author);
            fs::write(&path, user_file.to_string())?;
        }
        report.user_file_path = Some(path);
    }

    if !game_data.rich_presence().is_empty() {
        let path = rich_presence_path(game_data, &output);
        if !dry_run {
            fs::write(&path, game_data.rich_presence().to_string())?;
        }
        report.rich_presence_path = Some(path);
    }

    report.print(game_data, verbosity);
    Ok(())
}

/// Computes the user file path.
fn user_file_path(game_data: &GameData, output: &Path) -> PathBuf {
    let filename = format!("{}{USER_FILE_SUFFIX}.{USER_FILE_EXTENSION}", game_data.id());
    output.join(filename)
}

/// Computes the rich presence file path.
fn rich_presence_path(game_data: &GameData, output: &Path) -> PathBuf {
    let filename = format!(
        "{}{RICH_PRESENCE_FILE_SUFFIX}.{RICH_PRESENCE_FILE_EXTENSION}",
        game_data.id()
    );
    output.join(filename)
}

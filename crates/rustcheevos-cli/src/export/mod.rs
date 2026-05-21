//! Export game assets to disk.

mod report;

use std::fs;
use std::path::{Path, PathBuf};

use rustcheevos::types::game::GameData;
use rustcheevos_schema::rich::{RICH_PRESENCE_FILE_EXTENSION, RICH_PRESENCE_FILE_SUFFIX};
use rustcheevos_schema::user::{USER_FILE_EXTENSION, USER_FILE_SUFFIX};

use crate::CliError;
use crate::Verbosity;

use self::report::ExportReport;

/// Export game assets to disk.
///
/// # Errors
/// Returns an error if the export fails.
pub fn export(
    game_data: &GameData,
    output: &Path,
    author: String,
    verbosity: Verbosity,
    dry_run: bool,
) -> Result<(), CliError> {
    if !dry_run {
        fs::create_dir_all(output)?;
    }

    let mut report = ExportReport::new(dry_run);

    let has_user_file = !game_data.achievements().is_empty()
        || !game_data.leaderboards().is_empty()
        || !game_data.code_notes().is_empty();
    if has_user_file {
        let path = user_file_path(game_data, output);
        if !dry_run {
            let user_file = game_data.to_user_file(author);
            fs::write(&path, user_file.to_string())?;
        }
        report.user_file_path = Some(path);
    }

    if !game_data.rich_presence().is_empty() {
        let path = rich_presence_path(game_data, output);
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

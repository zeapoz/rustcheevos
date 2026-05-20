//! Export game assets to disk.

use std::fs;
use std::path::{Path, PathBuf};

use rustcheevos::types::game::GameData;
use rustcheevos_schema::rich::{RICH_PRESENCE_FILE_EXTENSION, RICH_PRESENCE_FILE_SUFFIX};
use rustcheevos_schema::user::{USER_FILE_EXTENSION, USER_FILE_SUFFIX};

use crate::CliError;

/// Export game assets to disk.
///
/// # Errors
/// Returns an error if the export fails.
pub fn export(game_data: &GameData, output: &Path) -> Result<(), CliError> {
    fs::create_dir_all(output)?;

    let achievements = game_data.achievements().len();
    let leaderboards = game_data.leaderboards().len();
    let code_notes = game_data.code_notes().len();
    let has_rich_presence = !game_data.rich_presence().is_empty();

    let has_user_file = achievements > 0 || leaderboards > 0 || code_notes > 0;
    if has_user_file {
        let path = export_user_file(game_data, output)?;

        let total = achievements + leaderboards + code_notes;
        println!("Exported {total} game assets to {}", path.display());

        if achievements > 0 {
            println!("- {achievements} achievement(s)");
        }
        if leaderboards > 0 {
            println!("- {leaderboards} leaderboard(s)");
        }
        if code_notes > 0 {
            println!("- {code_notes} code note(s)");
        }
    }

    if has_rich_presence {
        let path = export_rich_presence(game_data, output)?;
        println!("Exported rich presence to {}", path.display());
    }

    Ok(())
}

/// Exports the user file.
fn export_user_file(game_data: &GameData, output: &Path) -> Result<PathBuf, CliError> {
    let filename = format!("{}{USER_FILE_SUFFIX}.{USER_FILE_EXTENSION}", game_data.id());
    let path = output.join(filename);
    let user_file = game_data.to_user_file();
    fs::write(&path, user_file.to_string())?;
    Ok(path)
}

/// Exports the rich presence file.
fn export_rich_presence(game_data: &GameData, output: &Path) -> Result<PathBuf, CliError> {
    let filename = format!(
        "{}{RICH_PRESENCE_FILE_SUFFIX}.{RICH_PRESENCE_FILE_EXTENSION}",
        game_data.id()
    );
    let path = output.join(filename);
    fs::write(&path, game_data.rich_presence().to_string())?;
    Ok(path)
}

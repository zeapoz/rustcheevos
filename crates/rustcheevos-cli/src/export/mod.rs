//! Export game assets to disk.

use std::path::Path;

use rustcheevos::types::game::GameData;

use crate::CliError;

/// Export game assets to disk.
///
/// # Errors
/// Returns an error if the export fails.
pub fn export(game_data: &GameData, output: &Path) -> Result<(), CliError> {
    let exported = game_data.export(output)?;

    let achievements = game_data.achievements().len();
    let leaderboards = game_data.leaderboards().len();
    let code_notes = game_data.code_notes().len();

    if let Some(ref path) = exported.user_file {
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

    if let Some(ref path) = exported.rich_presence {
        println!("Exported rich presence to {}", path.display());
    }
    Ok(())
}

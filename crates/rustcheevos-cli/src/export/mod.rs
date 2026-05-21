//! Export game assets to disk.

use std::fs;
use std::path::{Path, PathBuf};

use rustcheevos::types::achievement::Achievement;
use rustcheevos::types::game::GameData;
use rustcheevos::types::leaderboard::Leaderboard;
use rustcheevos::types::note::CodeNote;
use rustcheevos_schema::rich::{RICH_PRESENCE_FILE_EXTENSION, RICH_PRESENCE_FILE_SUFFIX};
use rustcheevos_schema::user::{USER_FILE_EXTENSION, USER_FILE_SUFFIX};

use crate::CliError;
use crate::Verbosity;

/// Collects export state and handles output formatting.
#[derive(Default)]
struct ExportReport {
    /// Path to the exported user file, if any.
    user_file_path: Option<PathBuf>,
    /// Path to the exported rich presence file, if any.
    rich_presence_path: Option<PathBuf>,
}

impl ExportReport {
    /// Prints the export report at the given verbosity level.
    fn print(&self, game_data: &GameData, verbosity: Verbosity) {
        match verbosity {
            Verbosity::Quiet => {}
            Verbosity::Normal => {
                self.print_user_file(game_data, true);
                self.print_rich_presence();
            }
            Verbosity::Verbose => {
                Self::print_achievements(game_data.achievements());
                Self::print_leaderboards(game_data.leaderboards());
                Self::print_code_notes(game_data.code_notes());
                self.print_user_file(game_data, false);
                self.print_rich_presence();
            }
        }
    }

    /// Prints rich presence export path.
    fn print_rich_presence(&self) {
        if let Some(path) = &self.rich_presence_path {
            println!("Exported rich presence to {}", path.display());
        }
    }

    /// Prints achievement details.
    fn print_achievements(achievements: &[Achievement]) {
        if achievements.is_empty() {
            return;
        }
        println!("Exported {} achievement(s):", achievements.len());
        for ach in achievements {
            println!(
                "- {}: {} - {} ({})",
                ach.id(),
                ach.title(),
                ach.description(),
                ach.points()
            );
        }
        println!();
    }

    /// Prints leaderboard details.
    fn print_leaderboards(leaderboards: &[Leaderboard]) {
        if leaderboards.is_empty() {
            return;
        }
        println!("Exported {} leaderboard(s):", leaderboards.len());
        for lb in leaderboards {
            println!(
                "- {}: {} - {} ({})",
                lb.id(),
                lb.title(),
                lb.description(),
                lb.format().as_str()
            );
        }
        println!();
    }

    /// Prints code note details.
    fn print_code_notes(code_notes: &[CodeNote]) {
        if code_notes.is_empty() {
            return;
        }
        println!("Exported {} code note(s):", code_notes.len());
        for note in code_notes {
            let title = note.contents().lines().next().unwrap_or("");
            println!("- 0x{:04X}: {title}", note.address());
        }
        println!();
    }

    /// Prints the export summary.
    fn print_user_file(&self, game_data: &GameData, show_breakdown: bool) {
        if let Some(path) = &self.user_file_path {
            let achievements = game_data.achievements().len();
            let leaderboards = game_data.leaderboards().len();
            let code_notes = game_data.code_notes().len();
            let total = achievements + leaderboards + code_notes;
            println!("Exported {total} game assets to {}", path.display());
            if show_breakdown {
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
        }
    }
}

/// Export game assets to disk.
///
/// # Errors
/// Returns an error if the export fails.
pub fn export(
    game_data: &GameData,
    output: &Path,
    author: String,
    verbosity: Verbosity,
) -> Result<(), CliError> {
    fs::create_dir_all(output)?;

    let mut report = ExportReport::default();

    let has_user_file = !game_data.achievements().is_empty()
        || !game_data.leaderboards().is_empty()
        || !game_data.code_notes().is_empty();
    if has_user_file {
        let path = export_user_file(game_data, output, author)?;
        report.user_file_path = Some(path);
    }

    if !game_data.rich_presence().is_empty() {
        let path = export_rich_presence(game_data, output)?;
        report.rich_presence_path = Some(path);
    }

    report.print(game_data, verbosity);
    Ok(())
}

/// Exports the user file.
fn export_user_file(
    game_data: &GameData,
    output: &Path,
    author: String,
) -> Result<PathBuf, CliError> {
    let filename = format!("{}{USER_FILE_SUFFIX}.{USER_FILE_EXTENSION}", game_data.id());
    let path = output.join(filename);
    let user_file = game_data.to_user_file(author);
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

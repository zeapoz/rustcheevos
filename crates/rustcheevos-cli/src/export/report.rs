//! Export report for tracking and printing export state.

use std::path::PathBuf;

use rustcheevos::types::achievement::Achievement;
use rustcheevos::types::game::GameData;
use rustcheevos::types::leaderboard::Leaderboard;
use rustcheevos::types::note::CodeNote;

use crate::Verbosity;

/// Returns the verb for export messages based on dry-run mode.
fn export_verb(dry_run: bool) -> &'static str {
    if dry_run { "Would export" } else { "Exported" }
}

/// Collects export state and handles output formatting.
#[derive(Default)]
pub struct ExportReport {
    /// Path to the exported user file, if any.
    pub user_file_path: Option<PathBuf>,
    /// Path to the exported rich presence file, if any.
    pub rich_presence_path: Option<PathBuf>,
    /// Whether this is a dry run (no files written).
    pub dry_run: bool,
}

impl ExportReport {
    /// Creates a new export report with the given dry-run mode.
    pub fn new(dry_run: bool) -> Self {
        Self {
            dry_run,
            ..Default::default()
        }
    }

    /// Prints the export report at the given verbosity level.
    pub fn print(&self, game_data: &GameData, verbosity: Verbosity) {
        match verbosity {
            Verbosity::Quiet => {}
            Verbosity::Normal => {
                self.print_user_file(game_data, true);
                self.print_rich_presence();
            }
            Verbosity::Verbose => {
                Self::print_achievements(game_data.achievements(), self.dry_run);
                Self::print_leaderboards(game_data.leaderboards(), self.dry_run);
                Self::print_code_notes(game_data.code_notes(), self.dry_run);
                self.print_user_file(game_data, false);
                self.print_rich_presence();
            }
        }
    }

    /// Prints rich presence export path.
    fn print_rich_presence(&self) {
        if let Some(path) = &self.rich_presence_path {
            println!(
                "{} rich presence to {}",
                export_verb(self.dry_run),
                path.display()
            );
        }
    }

    /// Prints achievement details.
    fn print_achievements(achievements: &[Achievement], dry_run: bool) {
        if achievements.is_empty() {
            return;
        }
        println!(
            "{} {} achievement(s):",
            export_verb(dry_run),
            achievements.len()
        );
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
    fn print_leaderboards(leaderboards: &[Leaderboard], dry_run: bool) {
        if leaderboards.is_empty() {
            return;
        }
        println!(
            "{} {} leaderboard(s):",
            export_verb(dry_run),
            leaderboards.len()
        );
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
    fn print_code_notes(code_notes: &[CodeNote], dry_run: bool) {
        if code_notes.is_empty() {
            return;
        }
        println!(
            "{} {} code note(s):",
            export_verb(dry_run),
            code_notes.len()
        );
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
            println!(
                "{} {total} game assets to {}",
                export_verb(self.dry_run),
                path.display()
            );
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

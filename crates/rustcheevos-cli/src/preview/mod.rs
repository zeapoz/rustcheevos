//! Preview generated game assets.

use clap::{Parser, Subcommand};
use rustcheevos::types::achievement::Achievement;
use rustcheevos::types::game::GameData;
use rustcheevos::types::leaderboard::Leaderboard;
use rustcheevos::types::note::CodeNote;
use rustcheevos::util::parse_hex_address;

use crate::preview::assets::achievement::AchievementPreview;
use crate::preview::assets::leaderboard::LeaderboardPreview;
use crate::preview::assets::note::CodeNotePreview;

mod assets;
mod table;

/// Arguments for preview command.
#[derive(Parser, Debug, Clone, Default)]
pub struct PreviewArgs {
    /// Which asset to preview. Shows all types if omitted.
    #[command(subcommand)]
    pub target: Option<PreviewTarget>,
}

/// The type of asset to preview.
#[derive(Subcommand, Debug, Clone)]
pub enum PreviewTarget {
    /// Preview an achievement by ID or title.
    Achievement {
        /// Numeric ID to match.
        #[arg(short, long, conflicts_with = "title")]
        id: Option<u32>,
        /// Title substring to match.
        #[arg(short, long, conflicts_with = "id")]
        title: Option<String>,
    },
    /// Preview a leaderboard by ID or title.
    Leaderboard {
        /// Numeric ID to match.
        #[arg(short, long, conflicts_with = "title")]
        id: Option<u32>,
        /// Title substring to match.
        #[arg(short, long, conflicts_with = "id")]
        title: Option<String>,
    },
    /// Preview a code note by address or text.
    Note {
        /// Hex address (e.g. 0x1234) to match.
        #[arg(short, long, conflicts_with = "text")]
        address: Option<String>,
        /// Content substring to match.
        #[arg(short, long, conflicts_with = "address")]
        text: Option<String>,
    },
    /// Preview the rich presence script.
    RichPresence,
}

/// Preview the output of a given game asset.
pub fn preview_output(game_data: &GameData, args: &PreviewArgs) {
    match &args.target {
        None => {
            preview_achievements(game_data, None, None);
            preview_leaderboards(game_data, None, None);
            preview_notes(game_data, None, None);
            preview_rich_presence(game_data);
        }
        Some(PreviewTarget::Achievement { id, title }) => {
            preview_achievements(game_data, *id, title.as_deref());
        }
        Some(PreviewTarget::Leaderboard { id, title }) => {
            preview_leaderboards(game_data, *id, title.as_deref());
        }
        Some(PreviewTarget::Note { address, text }) => {
            preview_notes(game_data, address.as_deref(), text.as_deref());
        }
        Some(PreviewTarget::RichPresence) => {
            preview_rich_presence(game_data);
        }
    }
}

/// Default width for separator lines.
const SEPARATOR_WIDTH: usize = 80;

/// Build a separator string with a left-aligned label.
pub(crate) fn format_separator(label: &str) -> String {
    let text = format!("── {label} ");
    let count = text.chars().count();
    let pad = SEPARATOR_WIDTH.saturating_sub(count);
    format!("{text}{}", "─".repeat(pad))
}

/// Print a horizontal rule separator with a left-aligned label.
fn print_separator(label: &str) {
    println!("{}", format_separator(label));
}

/// Report that no matching assets were found.
fn report_no_match(asset: &str, filter: &str, value: &str) {
    eprintln!("No {asset} found matching {filter}: {value}");
}

/// Display all matched achievements.
fn preview_achievements(data: &GameData, id: Option<u32>, title: Option<&str>) {
    let items = find_achievements(data.achievements(), id, title);
    let count = items.len();
    for (i, ach) in items.iter().enumerate() {
        print!("{}", AchievementPreview(ach));
        if i + 1 < count {
            println!();
        }
    }
}

/// Display all matched leaderboards.
fn preview_leaderboards(data: &GameData, id: Option<u32>, title: Option<&str>) {
    let items = find_leaderboards(data.leaderboards(), id, title);
    let count = items.len();
    for (i, lb) in items.iter().enumerate() {
        print!("{}", LeaderboardPreview(lb));
        if i + 1 < count {
            println!();
        }
    }
}

/// Display all matched code notes.
fn preview_notes(data: &GameData, address: Option<&str>, text: Option<&str>) {
    let items = find_notes(data.code_notes(), address, text);
    let count = items.len();
    for (i, note) in items.iter().enumerate() {
        print!("{}", CodeNotePreview(note));
        if i + 1 < count {
            println!();
        }
    }
}

/// Display the rich presence script.
fn preview_rich_presence(data: &GameData) {
    print_separator("Rich Presence");
    print!("{}", data.rich_presence());
}

/// Find items matching the given ID or title. Returns all if neither given.
fn find_by_id_title<'a, T>(
    items: &'a [T],
    id: Option<u32>,
    title: Option<&str>,
    asset: &str,
    get_id: impl Fn(&T) -> u32,
    get_title: impl Fn(&T) -> &str,
) -> Vec<&'a T> {
    let mut items: Vec<_> = items.iter().collect();

    if let Some(n) = id {
        items.retain(|a| get_id(a) == n);
    }

    if let Some(title_str) = title {
        let lower = title_str.to_lowercase();
        items.retain(|a| get_title(a).to_lowercase().contains(&lower));
    }

    if items.is_empty() {
        if let Some(n) = id {
            report_no_match(asset, "id", &n.to_string());
        } else if let Some(t) = title {
            report_no_match(asset, "title", t);
        }
    }

    items
}

/// Find achievements matching the given ID or title. Returns all if neither given.
fn find_achievements<'a>(
    achievements: &'a [Achievement],
    id: Option<u32>,
    title: Option<&str>,
) -> Vec<&'a Achievement> {
    find_by_id_title(
        achievements,
        id,
        title,
        "achievements",
        Achievement::id,
        |a| a.title(),
    )
}

/// Find leaderboards matching the given ID or title. Returns all if neither given.
fn find_leaderboards<'a>(
    leaderboards: &'a [Leaderboard],
    id: Option<u32>,
    title: Option<&str>,
) -> Vec<&'a Leaderboard> {
    find_by_id_title(
        leaderboards,
        id,
        title,
        "leaderboards",
        Leaderboard::id,
        |l| l.title(),
    )
}

/// Find code notes matching the given address or text. Returns all if neither given.
fn find_notes<'a>(
    notes: &'a [CodeNote],
    address: Option<&str>,
    text: Option<&str>,
) -> Vec<&'a CodeNote> {
    let mut items: Vec<_> = notes.iter().collect();

    if let Some(addr_str) = address {
        if let Ok(addr) = parse_hex_address(addr_str) {
            items.retain(|n| n.address() == addr);
        } else {
            report_no_match("notes", "address", addr_str);
            return Vec::new();
        }
    }

    if let Some(text_str) = text {
        let lower = text_str.to_lowercase();
        items.retain(|n| n.contents().to_lowercase().contains(&lower));
    }

    if items.is_empty() {
        if let Some(a) = address {
            report_no_match("notes", "address", a);
        } else if let Some(t) = text {
            report_no_match("notes", "text", t);
        }
    }

    items
}

#[cfg(test)]
mod tests {
    use super::format_separator;

    #[test]
    fn format_separator_pads_to_width() {
        let s = format_separator("Test");
        assert_eq!(s.chars().count(), 80);
        assert!(s.starts_with("── Test "));
        assert!(s.ends_with('─'));
    }

    #[test]
    fn format_separator_empty_label() {
        let s = format_separator("");
        assert_eq!(s.chars().count(), 80);
    }
}

//! Generate README files for Rustcheevos projects.

use std::{fmt::Write, fs, path::Path};

use rustcheevos::types::{
    achievement::{Achievement, Tag},
    game::GameData,
    leaderboard::{Leaderboard, LeaderboardFormat},
    rich::RichPresence,
};

use crate::CliError;
use hash::SupportedHash;

mod hash;

/// Generate a README file for the given game data.
///
/// # Errors
/// Returns an error if writing to the output path fails or if formatting fails.
pub fn generate_readme(
    game_data: &GameData,
    output: &Path,
    hashes_path: Option<&Path>,
) -> Result<(), CliError> {
    let hashes = hashes_path
        .map(SupportedHash::parse_from_file)
        .transpose()?;
    let content = build_readme(game_data, hashes.as_deref())?;
    fs::write(output, content)?;
    println!("Generated README at {}", output.display());
    Ok(())
}

/// Builds the full README content from game data.
fn build_readme(
    game_data: &GameData,
    hashes: Option<&[SupportedHash]>,
) -> Result<String, CliError> {
    let mut md = String::new();

    writeln!(md, "# {}\n", game_data.title())?;
    writeln!(
        md,
        "[View on RetroAchievements](https://retroachievements.org/game/{})\n",
        game_data.id()
    )?;

    let has_hashes = hashes.is_some_and(|h| !h.is_empty());
    let has_achievements = !game_data.achievements().is_empty();
    let has_leaderboards = !game_data.leaderboards().is_empty();
    let has_rich_presence = !game_data.rich_presence().is_empty();

    if has_hashes || has_achievements || has_leaderboards || has_rich_presence {
        writeln!(md, "## Table of Contents\n")?;
        let sections = [
            (has_hashes, "Supported Hashes"),
            (has_achievements, "Achievements"),
            (has_leaderboards, "Leaderboards"),
            (has_rich_presence, "Rich Presence"),
        ];
        for (visible, title) in sections {
            if visible {
                writeln!(
                    md,
                    "- [{title}](#{})",
                    title.to_lowercase().replace(' ', "-")
                )?;
            }
        }
        writeln!(md)?;
    }

    if let Some(hashes) = hashes.filter(|h| !h.is_empty()) {
        build_hashes_section(&mut md, hashes)?;
    }

    if has_achievements {
        build_achievements_section(&mut md, game_data.achievements())?;
    }

    if has_leaderboards {
        build_leaderboards_section(&mut md, game_data.leaderboards())?;
    }

    if has_rich_presence {
        build_rich_presence_section(&mut md, game_data.rich_presence())?;
    }

    Ok(md)
}

/// Builds the supported hashes section of the README.
fn build_hashes_section(md: &mut String, hashes: &[SupportedHash]) -> Result<(), CliError> {
    writeln!(md, "## Supported Hashes\n")?;

    writeln!(md, "| Hash | ROM |")?;
    writeln!(md, "|------|-----|")?;

    for hash in hashes {
        writeln!(md, "| `{}` | {} |", hash.hash(), hash.name())?;
    }

    writeln!(md)?;
    Ok(())
}

/// Builds the achievements section of the README.
fn build_achievements_section(
    md: &mut String,
    achievements: &[Achievement],
) -> Result<(), CliError> {
    let count = achievements.len();
    let total_points: u32 = achievements.iter().map(Achievement::points).sum();

    writeln!(md, "## Achievements\n")?;
    writeln!(
        md,
        "**{count} achievement{} worth {total_points} point{}**\n",
        if count == 1 { "" } else { "s" },
        if total_points == 1 { "" } else { "s" }
    )?;

    writeln!(md, "| Badge | Name | Description | Points | Tag |")?;
    writeln!(md, "|-------|------|-------------|--------|-----|")?;

    for achievement in achievements {
        let badge = if achievement.badge_id() > 0 {
            format!(
                "![{}](https://media.retroachievements.org/Badge/{}.png)",
                escape_cell(achievement.title()),
                achievement.badge_id()
            )
        } else {
            String::new()
        };
        let tag = match achievement.tag() {
            Some(Tag::Progression) => "Progression",
            Some(Tag::WinCondition) => "Win Condition",
            Some(Tag::Missable) => "Missable",
            None => "",
        };
        writeln!(
            md,
            "| {badge} | {} | {} | {} | {} |",
            escape_cell(achievement.title()),
            escape_cell(achievement.description()),
            achievement.points(),
            tag
        )?;
    }

    writeln!(md)?;
    Ok(())
}

/// Builds the leaderboards section of the README.
fn build_leaderboards_section(
    md: &mut String,
    leaderboards: &[Leaderboard],
) -> Result<(), CliError> {
    let count = leaderboards.len();

    writeln!(md, "## Leaderboards\n")?;
    writeln!(
        md,
        "**{count} leaderboard{}**\n",
        if count == 1 { "" } else { "s" }
    )?;

    writeln!(md, "| Name | Format |")?;
    writeln!(md, "|------|--------|")?;

    for leaderboard in leaderboards {
        let format = match leaderboard.format() {
            LeaderboardFormat::Score => "Score",
            LeaderboardFormat::Seconds => "Seconds",
            LeaderboardFormat::Frames => "Frames",
            LeaderboardFormat::Milliseconds => "Milliseconds",
            LeaderboardFormat::Minutes => "Minutes",
            LeaderboardFormat::SecsAsMins => "SecsAsMins",
            LeaderboardFormat::Value => "Value",
            LeaderboardFormat::Unsigned => "Unsigned",
            LeaderboardFormat::Tens => "Tens",
            LeaderboardFormat::Hundreds => "Hundreds",
            LeaderboardFormat::Thousands => "Thousands",
            LeaderboardFormat::Fixed1 => "Fixed1",
            LeaderboardFormat::Fixed2 => "Fixed2",
            LeaderboardFormat::Fixed3 => "Fixed3",
            LeaderboardFormat::Points => "Points",
            LeaderboardFormat::Custom => "Custom",
        };
        writeln!(md, "| {} | {format} |", escape_cell(leaderboard.title()))?;
    }

    writeln!(md)?;
    Ok(())
}

/// Builds the rich presence section of the README.
fn build_rich_presence_section(md: &mut String, rich: &RichPresence) -> Result<(), CliError> {
    writeln!(md, "## Rich Presence\n")?;

    writeln!(md, "| Display |")?;
    writeln!(md, "|---------|")?;

    for display in rich.iter_conditional_displays() {
        writeln!(md, "| `{}` |", clean_display(display.display()))?;
    }

    let static_display = rich.static_display();
    if !static_display.is_empty() {
        writeln!(md, "| `{}` |", clean_display(static_display))?;
    }

    writeln!(md)?;
    Ok(())
}

/// Escapes a string for safe use in a markdown table cell.
fn escape_cell(s: &str) -> String {
    s.replace('|', "\\|")
}

/// Cleans a rich presence display string by stripping macro argument details
/// and escaping pipe characters for markdown table rendering.
fn clean_display(display: &str) -> String {
    let mut result = String::with_capacity(display.len());
    let mut chars = display.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '@' {
            result.push('[');
            // Copy the macro name
            while let Some(&next) = chars.peek() {
                if next == '(' || next.is_whitespace() || next == '|' {
                    break;
                }
                chars.next();
                result.push(next);
            }
            result.push(']');
            // Skip the (...) block
            if chars.peek() == Some(&'(') {
                chars.next();
                let mut depth = 1;
                while let Some(&next) = chars.peek() {
                    chars.next();
                    if next == '(' {
                        depth += 1;
                    } else if next == ')' {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                }
            }
        } else if c == '|' {
            result.push_str("\\|");
        } else {
            result.push(c);
        }
    }

    result
}

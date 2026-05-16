//! Definitions for user files

use std::fmt;

use thiserror::Error;

use header::Header;

use crate::prelude::{Achievement, CodeNote, Leaderboard};

mod header;

/// The suffix for user files.
pub const USER_FILE_SUFFIX: &str = "-User";
/// The file extension for user files.
pub const USER_FILE_EXTENSION: &str = "txt";

/// The author to use when none is specified.
const DEFAULT_AUTHOR: &str = "rustcheevos";
/// The timestamp to use when none is specified.
const DEFAULT_TIMESTAMP: &str = "0";
/// The badge ID to use when none is specified.
const DEFAULT_BADGE_ID: &str = "00000";

/// The error type for user file parsing.
#[derive(Error, Debug, Clone)]
pub enum ParseError {
    /// The protocol version is invalid.
    #[error("invalid protocol version: {0}")]
    InvalidProtocolVersion(String),
    /// The header is invalid.
    #[error("invalid header: {0}")]
    InvalidHeader(String),
}

/// The user file schema.
#[derive(Debug, Clone)]
pub struct UserFile {
    /// The header of the user file.
    pub header: Header,
    /// The achievement entries of the user file.
    pub achievements: Vec<AchievementEntry>,
    /// The leaderboard entries of the user file.
    pub leaderboards: Vec<LeaderboardEntry>,
    /// The code note entries of the user file.
    pub notes: Vec<CodeNoteEntry>,
}

impl UserFile {
    /// Creates and returns a new user file.
    pub fn new(
        game_title: impl Into<String>,
        achievements: impl IntoIterator<Item = impl Into<AchievementEntry>>,
        leaderboards: impl IntoIterator<Item = impl Into<LeaderboardEntry>>,
        notes: impl IntoIterator<Item = impl Into<CodeNoteEntry>>,
    ) -> Self {
        Self {
            header: Header::new(game_title),
            achievements: achievements.into_iter().map(Into::into).collect(),
            leaderboards: leaderboards.into_iter().map(Into::into).collect(),
            notes: notes.into_iter().map(Into::into).collect(),
        }
    }
}

impl fmt::Display for UserFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.header)?;
        for achievement in &self.achievements {
            writeln!(f, "{achievement}")?;
        }
        for leaderboard in &self.leaderboards {
            writeln!(f, "{leaderboard}")?;
        }
        for note in &self.notes {
            writeln!(f, "{note}")?;
        }
        Ok(())
    }
}

/// An achievement entry in a user file.
#[derive(Debug, Clone, PartialEq)]
pub struct AchievementEntry {
    /// The achievement ID.
    pub id: u32,
    /// The requirements for the achievement.
    pub requirements: String,
    /// The achievement title.
    pub title: String,
    /// The achievement description.
    pub description: String,
    /// The tag for the achive  
    pub tag: String,
    /// The author of the achievement.
    pub author: String,
    /// The number of points for the achievement.
    pub points: u32,
    /// The date the achievement was created.
    pub created: String,
    /// The date the achievement was last updated.
    pub updated: String,
    /// The number of upvotes for the achievement, unused.
    pub upvotes: u32,
    /// The number of downvotes for the achievement, unused.
    pub downvotes: u32,
    /// The link to the badge icon for the achievement.
    pub badge: String,
}

impl fmt::Display for AchievementEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"{}:"{}":"{}":"{}": : :{}:{}:{}:{}:{}:{}:{}:{}"#,
            self.id,
            self.requirements,
            self.title,
            self.description,
            self.tag,
            self.author,
            self.points,
            self.created,
            self.updated,
            self.upvotes,
            self.downvotes,
            self.badge,
        )
    }
}

impl From<&Achievement> for AchievementEntry {
    fn from(value: &Achievement) -> Self {
        Self {
            id: value.id,
            requirements: value.requirements.to_string(),
            title: value.title.clone(),
            description: value.description.clone(),
            tag: value
                .tag
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_default(),
            author: DEFAULT_AUTHOR.to_string(),
            points: value.points,
            created: DEFAULT_TIMESTAMP.to_string(),
            updated: DEFAULT_TIMESTAMP.to_string(),
            upvotes: 0,
            downvotes: 0,
            badge: value
                .badge_id
                .map_or_else(|| DEFAULT_BADGE_ID.to_string(), |id| format!("{id:05}")),
        }
    }
}

/// A leaderboard entry in a user file.
#[derive(Debug, Clone, PartialEq)]
pub struct LeaderboardEntry {
    /// The leaderboard ID.
    pub id: u32,
    /// The leaderboard start condition.
    pub start: String,
    /// The leaderboard cancel condition.
    pub cancel: String,
    /// The leaderboard submit condition.
    pub submit: String,
    /// The leaderboard value condition.
    pub value: String,
    /// The leaderboard format.
    pub format: String,
    /// The leaderboard title.
    pub title: String,
    /// The leaderboard description.
    pub description: String,
    /// Whether lower values are to be considered better.
    pub lower_is_better: bool,
}

impl fmt::Display for LeaderboardEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"L{}:"{}":"{}":"{}":"{}":{}:"{}":"{}":{}"#,
            self.id,
            self.start,
            self.cancel,
            self.submit,
            self.value,
            self.format,
            self.title,
            self.description,
            i32::from(self.lower_is_better)
        )
    }
}

impl From<&Leaderboard> for LeaderboardEntry {
    fn from(value: &Leaderboard) -> Self {
        Self {
            id: value.id,
            start: value.start.to_string(),
            cancel: value.cancel.to_string(),
            submit: value.submit.to_string(),
            value: value.value.to_string(),
            format: value.format.to_string(),
            title: value.title.clone(),
            description: value.description.clone(),
            lower_is_better: value.lower_is_better,
        }
    }
}

/// A code note entry in a user file.
#[derive(Debug, Clone, PartialEq)]
pub struct CodeNoteEntry {
    /// The address of the code note.
    pub address: usize,
    /// The note.
    pub note: String,
}

impl fmt::Display for CodeNoteEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "N0:0x{:x}:{}", self.address, self.note)
    }
}

impl From<&CodeNote> for CodeNoteEntry {
    fn from(value: &CodeNote) -> Self {
        Self {
            address: value.address,
            note: value.contents.clone(),
        }
    }
}

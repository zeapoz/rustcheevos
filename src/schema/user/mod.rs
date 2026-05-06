use std::fmt;

use header::Header;

use thiserror::Error;

use crate::prelude::{Achievement, Leaderboard, Set};

pub mod header;

/// The suffix for user files.
pub const USER_FILE_SUFFIX: &str = "-User";
/// The file extension for user files.
pub const USER_FILE_EXTENSION: &str = "txt";

/// The author to use when none is specified.
const DEFAULT_AUTHOR: &str = "Author";
/// The timestamp to use when none is specified.
const DEFAULT_TIMESTAMP: &str = "0";
/// The badge ID to use when none is specified.
const DEFAULT_BADGE_ID: &str = "00000";

#[derive(Error, Debug, Clone)]
pub enum ParseError {
    #[error("invalid protocol version: {0}")]
    InvalidProtocolVersion(String),
    #[error("invalid header: {0}")]
    InvalidHeader(String),
}

/// The user file schema.
#[derive(Debug, Clone)]
pub struct UserFile {
    pub header: Header,
    pub achievements: Vec<AchievementEntry>,
    pub leaderboards: Vec<LeaderboardEntry>,
    pub notes: Vec<CodeNote>,
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

impl From<Set> for UserFile {
    fn from(set: Set) -> Self {
        let header = Header::new(set.game_name);
        let achievements = set
            .achievements
            .into_iter()
            .map(AchievementEntry::from)
            .collect();
        let leaderboards = set
            .leaderboards
            .into_iter()
            .map(LeaderboardEntry::from)
            .collect();
        UserFile {
            header,
            achievements,
            leaderboards,
            notes: Vec::new(),
        }
    }
}

/// An achievement entry in a user file.
#[derive(Debug, Clone, PartialEq)]
pub struct AchievementEntry {
    pub id: u32,
    pub requirements: String,
    pub title: String,
    pub description: String,
    pub tag: String,
    pub author: String,
    pub points: u32,
    pub created: String,
    pub updated: String,
    pub upvotes: u32,
    pub downvotes: u32,
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

impl From<Achievement> for AchievementEntry {
    fn from(value: Achievement) -> Self {
        Self {
            id: value.id,
            requirements: value.serialize_requirements(),
            title: value.title,
            description: value.description,
            tag: value.tag.to_string(),
            author: DEFAULT_AUTHOR.to_string(),
            points: value.points,
            created: DEFAULT_TIMESTAMP.to_string(),
            updated: DEFAULT_TIMESTAMP.to_string(),
            upvotes: 0,
            downvotes: 0,
            badge: DEFAULT_BADGE_ID.to_string(),
        }
    }
}

/// A leaderboard entry in a user file.
#[derive(Debug, Clone, PartialEq)]
pub struct LeaderboardEntry {
    pub id: u32,
    pub start: String,
    pub cancel: String,
    pub submit: String,
    pub value: String,
    pub format: String,
    pub title: String,
    pub description: String,
    pub lower_is_better: bool,
}

impl fmt::Display for LeaderboardEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"L{}:"{}":"{}":"{}":"{}":{}:{}:{}:{}"#,
            self.id,
            self.start,
            self.cancel,
            self.submit,
            self.value,
            self.format,
            self.title,
            self.description,
            if self.lower_is_better { 1 } else { 0 }
        )
    }
}

impl From<Leaderboard> for LeaderboardEntry {
    fn from(value: Leaderboard) -> Self {
        Self {
            id: value.id,
            start: value.start.to_string(),
            cancel: value.cancel.to_string(),
            submit: value.submit.to_string(),
            value: value.value.to_string(),
            format: value.format.to_string(),
            title: value.title,
            description: value.description,
            lower_is_better: value.lower_is_better,
        }
    }
}

/// A code note in a user file.
#[derive(Debug, Clone, PartialEq)]
pub struct CodeNote {
    pub address: u32,
    pub note: String,
}

impl fmt::Display for CodeNote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"N0:0x{:x}:{}"#, self.address, self.note)
    }
}

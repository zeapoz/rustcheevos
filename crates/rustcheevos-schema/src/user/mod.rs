//! Definitions for user files

use std::fmt;

use thiserror::Error;

use header::Header;

mod header;

/// The suffix for user files.
pub const USER_FILE_SUFFIX: &str = "-User";
/// The file extension for user files.
pub const USER_FILE_EXTENSION: &str = "txt";

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
        achievements: impl IntoIterator<Item = AchievementEntry>,
        leaderboards: impl IntoIterator<Item = LeaderboardEntry>,
        notes: impl IntoIterator<Item = CodeNoteEntry>,
    ) -> Self {
        Self {
            header: Header::new(game_title),
            achievements: achievements.into_iter().collect(),
            leaderboards: leaderboards.into_iter().collect(),
            notes: notes.into_iter().collect(),
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

/// Escapes backslashes and double quotes for a quoted user file field.
///
/// Backslashes are escaped first so that a backslash preceding a quote cannot
/// merge with the quote's escape sequence and terminate the field early.
fn escape_quoted(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

impl fmt::Display for AchievementEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"{}:"{}":"{}":"{}": : :{}:{}:{}:{}:{}:{}:{}:{}"#,
            self.id,
            self.requirements,
            escape_quoted(&self.title),
            escape_quoted(&self.description),
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
            escape_quoted(&self.title),
            escape_quoted(&self.description),
            i32::from(self.lower_is_better)
        )
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
        let escaped = self.note.replace("\r\n", "\\n").replace('\n', "\\n");
        write!(f, "N0:0x{:x}:\"{}\"", self.address, escaped)
    }
}

#[cfg(test)]
mod tests {
    use super::{AchievementEntry, LeaderboardEntry};

    /// Creates an achievement entry with the given title and description.
    fn achievement(title: &str, description: &str) -> AchievementEntry {
        AchievementEntry {
            id: 12345,
            requirements: "0xH1000=1".to_string(),
            title: title.to_string(),
            description: description.to_string(),
            tag: String::new(),
            author: "author".to_string(),
            points: 5,
            created: "2024-01-01 00:00:00".to_string(),
            updated: "2024-01-01 00:00:00".to_string(),
            upvotes: 0,
            downvotes: 0,
            badge: "01234".to_string(),
        }
    }

    /// Creates a leaderboard entry with the given title and description.
    fn leaderboard(title: &str, description: &str) -> LeaderboardEntry {
        LeaderboardEntry {
            id: 600_707,
            start: "0xH1000=1".to_string(),
            cancel: "0xH1000=0".to_string(),
            submit: "0xH1000=2".to_string(),
            value: "0xH2000".to_string(),
            format: "TIME".to_string(),
            title: title.to_string(),
            description: description.to_string(),
            lower_is_better: true,
        }
    }

    #[test]
    fn escape_quoted_escapes_quotes() {
        assert_eq!(super::escape_quoted(r#"He said "hi""#), r#"He said \"hi\""#);
    }

    #[test]
    fn escape_quoted_escapes_backslashes_first() {
        assert_eq!(super::escape_quoted(r"C:\save"), "C:\\\\save");
        assert_eq!(super::escape_quoted(r#"quote: ""#), r#"quote: \""#);
        assert_eq!(super::escape_quoted(r#"C:\dir""#), "C:\\\\dir\\\"");
    }

    #[test]
    fn escape_quoted_leaves_plain_text_unchanged() {
        assert_eq!(super::escape_quoted("Plain title"), "Plain title");
        assert_eq!(super::escape_quoted(""), "");
        assert_eq!(super::escape_quoted("Colon: fine"), "Colon: fine");
    }

    #[test]
    fn achievement_entry_escapes_title_and_description() {
        let entry = achievement(r#"Beat "Hard" mode"#, r"C:\save file");
        let line = entry.to_string();
        assert_eq!(
            line,
            r#"12345:"0xH1000=1":"Beat \"Hard\" mode":"C:\\save file": : ::author:5:2024-01-01 00:00:00:2024-01-01 00:00:00:0:0:01234"#
        );
    }

    #[test]
    fn achievement_entry_without_escapes_is_unchanged() {
        let entry = achievement("First Step", "Reach the flag");
        assert_eq!(
            entry.to_string(),
            r#"12345:"0xH1000=1":"First Step":"Reach the flag": : ::author:5:2024-01-01 00:00:00:2024-01-01 00:00:00:0:0:01234"#
        );
    }

    #[test]
    fn leaderboard_entry_escapes_title_and_description() {
        let entry = leaderboard(r#"Speed Run "Any%""#, "Finish in one go");
        assert_eq!(
            entry.to_string(),
            r#"L600707:"0xH1000=1":"0xH1000=0":"0xH1000=2":"0xH2000":TIME:"Speed Run \"Any%\"":"Finish in one go":1"#
        );
    }

    #[test]
    fn leaderboard_entry_without_escapes_is_unchanged() {
        let entry = leaderboard("Speed Run", "Complete the level");
        assert_eq!(
            entry.to_string(),
            r#"L600707:"0xH1000=1":"0xH1000=0":"0xH1000=2":"0xH2000":TIME:"Speed Run":"Complete the level":1"#
        );
    }
}

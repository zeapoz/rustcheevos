use std::str::FromStr;

use crate::types::ParseError as TypesParseError;
use crate::types::achievement::{Achievement, Tag};
use crate::types::set::Set;

const DEFAULT_AUTHOR: &str = "Author";
const PROTOCOL_VERSION: &str = "1.3";

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("invalid tag: {0}")]
    InvalidTag(String),

    #[error("types error: {0}")]
    Types(#[from] TypesParseError),

    #[error("invalid entry: {0}")]
    InvalidEntry(String),

    #[error("invalid leaderboard: {0}")]
    InvalidLeaderboard(String),

    #[error("invalid note: {0}")]
    InvalidNote(String),

    #[error("invalid header: {0}")]
    InvalidHeader(String),
}

#[derive(Debug, Clone)]
pub struct Header {
    pub version: String,
    pub game_title: String,
}

impl FromStr for Header {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((version, title)) = s.split_once('\n') {
            Ok(Header {
                version: version.to_string(),
                game_title: title.trim().to_string(),
            })
        } else {
            Err(ParseError::InvalidHeader("invalid header format".into()))
        }
    }
}

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.version, self.game_title)
    }
}

#[derive(Debug, Clone)]
pub struct UserFile {
    pub header: Header,
    pub achievements: Vec<AchievementEntry>,
    pub leaderboards: Vec<LeaderboardEntry>,
    pub notes: Vec<CodeNote>,
}

#[derive(Debug, Clone)]
pub struct AchievementEntry {
    pub id: u32,
    pub condition: String,
    pub title: String,
    pub description: String,
    pub tag: Tag,
    pub author: String,
    pub points: u32,
    pub created: String,
    pub updated: String,
    pub upvotes: u32,
    pub downvotes: u32,
    pub badge: String,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct CodeNote {
    pub address: u32,
    pub note: String,
}

impl FromStr for UserFile {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let header = format!(
            "{}\n{}",
            lines
                .next()
                .ok_or(ParseError::InvalidEntry("empty file".into()))?,
            lines
                .next()
                .ok_or(ParseError::InvalidEntry("missing game title".into()))?
        )
        .parse()?;

        let mut achievements = Vec::new();
        let mut leaderboards = Vec::new();
        let mut notes = Vec::new();

        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if let Some(c) = line.chars().next() {
                if c.is_ascii_digit() {
                    achievements.push(line.parse()?);
                } else if c == 'L' {
                    leaderboards.push(line.parse()?);
                } else if c == 'N' {
                    notes.push(line.parse()?);
                }
            }
        }

        Ok(UserFile {
            header,
            achievements,
            leaderboards,
            notes,
        })
    }
}

impl std::fmt::Display for UserFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

fn split_unquoted(s: &str, delim: char) -> Vec<&str> {
    let mut result = Vec::new();
    let mut in_quotes = false;
    let mut start = 0;

    for (i, c) in s.char_indices() {
        if c == '"' {
            in_quotes = !in_quotes;
        } else if c == delim && !in_quotes {
            result.push(&s[start..i]);
            start = i + 1;
        }
    }
    result.push(&s[start..]);
    result
}

impl FromStr for AchievementEntry {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<&str> = split_unquoted(s, ':');
        if fields.len() < 11 {
            return Err(ParseError::InvalidEntry(format!(
                "insufficient fields: {}",
                fields.len()
            )));
        }

        let id = fields[0]
            .parse()
            .map_err(|_| ParseError::InvalidEntry("invalid id".into()))?;

        let condition = fields[1].trim_matches('"').to_string();
        let title = fields[2].trim_matches('"').to_string();
        let description = fields[3].trim_matches('"').to_string();
        let tag = fields.get(6).map(|s| s.trim()).unwrap_or("").parse()?;
        let author = fields.get(7).map(|s| s.to_string()).unwrap_or_default();
        let points = fields
            .get(8)
            .ok_or_else(|| ParseError::InvalidEntry("invalid points".into()))?
            .parse()
            .map_err(|_| ParseError::InvalidEntry("invalid points".into()))?;
        let created = fields.get(9).map(|s| s.to_string()).unwrap_or_default();
        let updated = fields.get(10).map(|s| s.to_string()).unwrap_or_default();
        let upvotes = fields
            .get(11)
            .ok_or_else(|| ParseError::InvalidEntry("invalid upvotes".into()))?
            .parse()
            .map_err(|_| ParseError::InvalidEntry("invalid upvotes".into()))?;
        let downvotes = fields
            .get(12)
            .ok_or_else(|| ParseError::InvalidEntry("invalid downvotes".into()))?
            .parse()
            .map_err(|_| ParseError::InvalidEntry("invalid downvotes".into()))?;
        let badge = fields.get(13).map(|s| s.to_string()).unwrap_or_default();

        Ok(AchievementEntry {
            id,
            condition,
            title,
            description,
            tag,
            author,
            points,
            created,
            updated,
            upvotes,
            downvotes,
            badge,
        })
    }
}

impl std::fmt::Display for AchievementEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:\"{}\":\"{}\":\"{}\": : :{}:{}:{}:{}:{}:{}:{}:{}",
            self.id,
            self.condition,
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
    fn from(achievement: Achievement) -> Self {
        let condition = achievement.conditions.to_string();
        let tag = achievement.tag;
        let title = achievement.title;
        let description = achievement.description;
        let id = achievement.id.unwrap_or(0);
        let points = achievement.points;

        AchievementEntry {
            id,
            condition,
            title,
            description,
            tag,
            author: DEFAULT_AUTHOR.to_string(),
            points,
            created: "0".to_string(),
            updated: "0".to_string(),
            upvotes: 0,
            downvotes: 0,
            badge: "00000".to_string(),
        }
    }
}

impl From<Set> for UserFile {
    fn from(set: Set) -> Self {
        let header = Header {
            version: PROTOCOL_VERSION.to_string(),
            game_title: set.game_name,
        };
        let achievements = set
            .achievements
            .into_iter()
            .map(AchievementEntry::from)
            .collect();
        UserFile {
            header,
            achievements,
            leaderboards: Vec::new(),
            notes: Vec::new(),
        }
    }
}

impl FromStr for LeaderboardEntry {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<&str> = split_unquoted(s, ':');
        if fields.len() < 9 {
            return Err(ParseError::InvalidLeaderboard("insufficient fields".into()));
        }

        let id_part = fields[0].trim_start_matches('L');
        let id: u32 = id_part
            .split('|')
            .next()
            .ok_or_else(|| ParseError::InvalidLeaderboard("invalid id".into()))?
            .parse()
            .map_err(|_| ParseError::InvalidLeaderboard("invalid id".into()))?;

        let start = fields[1].trim_matches('"').to_string();
        let cancel = fields[2].trim_matches('"').to_string();
        let submit = fields[3].trim_matches('"').to_string();
        let value = fields[4].trim_matches('"').to_string();
        let format = fields[5].trim_matches('"').to_string();
        let title = fields[6].trim_matches('"').to_string();
        let description = fields[7].trim_matches('"').to_string();
        let lower_is_better = fields.get(8).map(|s| *s == "1").unwrap_or(false);

        Ok(LeaderboardEntry {
            id,
            start,
            cancel,
            submit,
            value,
            format,
            title,
            description,
            lower_is_better,
        })
    }
}

impl std::fmt::Display for LeaderboardEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "L{}:\"{}\":\"{}\":\"{}\":\"{}\":{}:\"{}\":\"{}\":{}",
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

impl FromStr for CodeNote {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix("N0:")
            .ok_or(ParseError::InvalidNote("missing N0: prefix".into()))?;
        let parts: Vec<&str> = s.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(ParseError::InvalidNote("missing note text".into()));
        }

        let address_str = parts[0].trim();
        let address = u32::from_str_radix(address_str.trim_start_matches("0x"), 16)
            .map_err(|_| ParseError::InvalidNote("invalid address".into()))?;

        let note = parts[1].trim_matches('"').to_string();

        Ok(CodeNote { address, note })
    }
}

impl std::fmt::Display for CodeNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "N0:0x{:x}:\"{}\"", self.address, self.note)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::achievement::{Achievement, Conditions, Tag};
    use crate::types::set::Set;

    #[test]
    fn test_parse_header() {
        let input = "1.3\nGeometry Wars: Galaxies";
        let header: Header = input.parse().unwrap();
        assert_eq!(header.version, "1.3");
        assert_eq!(header.game_title, "Geometry Wars: Galaxies");
    }

    #[test]
    fn test_parse_achievement() {
        let input = "600707:\"I:0xH1a8c94*2_0xU1a9c4d>=1\":\"Alpha Amateur \":\"Earn a Bronze medal or higher on every planet of the Alpha galaxy\": : :progression:Author:3:0:0:0:0:00000";
        let entry: AchievementEntry = input.parse().unwrap();
        assert_eq!(entry.id, 600707);
        assert_eq!(entry.title, "Alpha Amateur ");
        assert_eq!(
            entry.description,
            "Earn a Bronze medal or higher on every planet of the Alpha galaxy"
        );
        assert_eq!(entry.tag, Tag::Progression);
        assert_eq!(entry.author, "Author");
        assert_eq!(entry.points, 3);
        assert_eq!(entry.upvotes, 0);
        assert_eq!(entry.downvotes, 0);
    }

    #[test]
    fn test_parse_leaderboard() {
        let input = "L0:\"start_cond\":\"cancel_cond\":\"submit_cond\":\"M:0xX31d048\":VALUE:\"Title\":\"Desc\":0";
        let entry: LeaderboardEntry = input.parse().unwrap();
        assert_eq!(entry.id, 0);
        assert_eq!(entry.start, "start_cond");
        assert_eq!(entry.cancel, "cancel_cond");
        assert_eq!(entry.submit, "submit_cond");
        assert_eq!(entry.value, "M:0xX31d048");
        assert_eq!(entry.format, "VALUE");
        assert_eq!(entry.title, "Title");
        assert_eq!(entry.description, "Desc");
        assert!(!entry.lower_is_better);
    }

    #[test]
    fn test_parse_code_note() {
        let input = "N0:0x0000C5:\"Note text\"";
        let note: CodeNote = input.parse().unwrap();
        assert_eq!(note.address, 0xC5);
        assert_eq!(note.note, "Note text");
    }

    #[test]
    fn test_parse_full_user_file() {
        let input = "1.3\nGeometry Wars: Galaxies\n\
600707:\"condition\":\"Alpha Amateur \":\"Description\": : :progression:Author:3:0:0:0:0:00000\n\
L0:\"start\":\"cancel\":\"submit\":\"value\":VALUE:\"Title\":\"Desc\":0\n\
N0:0x0001:\"Note\"";
        let file: UserFile = input.parse().unwrap();
        assert_eq!(file.header.version, "1.3");
        assert_eq!(file.header.game_title, "Geometry Wars: Galaxies");
        assert_eq!(file.achievements.len(), 1);
        assert_eq!(file.achievements[0].id, 600707);
        assert_eq!(file.leaderboards.len(), 1);
        assert_eq!(file.leaderboards[0].id, 0);
        assert_eq!(file.notes.len(), 1);
        assert_eq!(file.notes[0].address, 1);
    }

    #[test]
    fn test_header_roundtrip() {
        let header = Header {
            version: "1.3".into(),
            game_title: "Geometry Wars: Galaxies".into(),
        };
        let serialized = format!("{}", header);
        let parsed: Header = serialized.parse().unwrap();
        assert_eq!(parsed.version, header.version);
        assert_eq!(parsed.game_title, header.game_title);
    }

    #[test]
    fn test_achievement_entry_roundtrip() {
        let input = "600707:\"I:0xH1a8c94*2_0xU1a9fad>=2\":\"Alpha Amateur \":\"Description\": : :progression:Author:3:0:0:0:0:00000";
        let entry: AchievementEntry = input.parse().unwrap();
        let serialized = format!("{}", entry);
        let parsed: AchievementEntry = serialized.parse().unwrap();
        assert_eq!(parsed.id, entry.id);
        assert_eq!(parsed.condition, entry.condition);
        assert_eq!(parsed.title, entry.title);
        assert_eq!(parsed.description, entry.description);
        assert_eq!(parsed.tag, entry.tag);
        assert_eq!(parsed.author, entry.author);
        assert_eq!(parsed.points, entry.points);
    }

    #[test]
    fn test_leaderboard_entry_roundtrip() {
        let input = "L0:\"start\":\"cancel\":\"submit\":\"M:0xX31d048\":VALUE:\"Title\":\"Desc\":0";
        let entry: LeaderboardEntry = input.parse().unwrap();
        let serialized = format!("{}", entry);
        let parsed: LeaderboardEntry = serialized.parse().unwrap();
        assert_eq!(parsed.id, entry.id);
        assert_eq!(parsed.start, entry.start);
        assert_eq!(parsed.cancel, entry.cancel);
        assert_eq!(parsed.submit, entry.submit);
        assert_eq!(parsed.value, entry.value);
        assert_eq!(parsed.format, entry.format);
        assert_eq!(parsed.title, entry.title);
        assert_eq!(parsed.description, entry.description);
        assert_eq!(parsed.lower_is_better, entry.lower_is_better);
    }

    #[test]
    fn test_code_note_roundtrip() {
        let input = "N0:0x0001C5:\"Note text\"";
        let note: CodeNote = input.parse().unwrap();
        let serialized = format!("{}", note);
        let parsed: CodeNote = serialized.parse().unwrap();
        assert_eq!(parsed.address, note.address);
        assert_eq!(parsed.note, note.note);
    }

    #[test]
    fn test_user_file_roundtrip() {
        let input = "1.3\nGame Title\n600707:\"cond\":\"Title\":\"Desc\": : :progression:Author:3:0:0:0:0:00000\nL0:\"s\":\"c\":\"sub\":\"v\":VALUE:\"T\":\"D\":0\nN0:0x123:\"Note\"";
        let file: UserFile = input.parse().unwrap();
        let serialized = format!("{}", file);
        let parsed: UserFile = serialized.parse().unwrap();
        assert_eq!(parsed.header.version, file.header.version);
        assert_eq!(parsed.header.game_title, file.header.game_title);
        assert_eq!(parsed.achievements.len(), file.achievements.len());
        assert_eq!(parsed.leaderboards.len(), file.leaderboards.len());
        assert_eq!(parsed.notes.len(), file.notes.len());
    }

    #[test]
    fn test_achievement_set_to_user_file_roundtrip() {
        let condition_str = "I:0xH1a8c94*2_0xU1a9fad>=2_I:0xH1a8c94*2_0xU1a9fad!=4_I:0xH1a8c94*2_0xU1a9fdd>=2_I:0xH1a8c94*2_0xU1a9fdd!=4_I:0xH1a8c94*2_0xU1aa00d>=2_I:0xH1a8c94*2_0xU1aa00d!=4_I:0xH1a8c94*2_0xU1aa03d>=2_I:0xH1a8c94*2_0xU1aa03d!=4_I:0xH1a8c94*2_0xU1aa06d>=2_I:0xH1a8c94*2_0xU1aa06d!=4_I:0xH1a8c94*2_0xU1aa09d>=2_I:0xH1a8c94*2_0xU1aa09d!=4_I:0xH1a8c94*2_0xU1aa0cd>=2_I:0xH1a8c94*2_0xU1aa0cd!=4_d0xH1a6990=1_0xH1a6990=1SI:0xH1a8c94*2_d0xU1a9fad<2SI:0xH1a8c94*2_d0xU1a9fdd<2SI:0xH1a8c94*2_d0xU1aa00d<2SI:0xH1a8c94*2_d0xU1aa03d<2SI:0xH1a8c94*2_d0xU1aa06d<2SI:0xH1a8c94*2_d0xU1aa09d<2SI:0xH1a8c94*2_d0xU1aa0cd<2";
        let conditions: Conditions = condition_str.parse().unwrap();
        let achievement = Achievement::new(
            String::from("Test Achievement"),
            String::from("Test description"),
            conditions,
            5,
        );
        let mut set = Set::new(String::from("1.3"), String::from("Test Game"));
        set.push(achievement);
        let user_file = UserFile::from(set);
        let serialized = format!("{}", user_file);
        let parsed: UserFile = serialized.parse().unwrap();
        assert_eq!(parsed.header.version, "1.3");
        assert_eq!(parsed.header.game_title, "Test Game");
        assert_eq!(parsed.achievements.len(), 1);
        assert_eq!(parsed.achievements[0].title, "Test Achievement");
        assert_eq!(parsed.achievements[0].points, 5);
    }
}

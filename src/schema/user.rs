use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::types::ParseError as TypesParseError;
use crate::types::achievement::{Achievement, Tag};
use crate::types::leaderboard::Leaderboard;
use crate::types::set::Set;

const PROTOCOL_VERSION: &str = "1.3";
const DEFAULT_AUTHOR: &str = "Author";
const DEFAULT_COUNT: u32 = 0;
const DEFAULT_TIMESTAMP: &str = "0";
const DEFAULT_BADGE: &str = "00000";

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

impl Display for Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

impl UserFile {
    pub fn merge_with_existing(&mut self, existing: &UserFile) {
        let existing_by_id: HashMap<_, _> =
            existing.achievements.iter().map(|a| (a.id, a)).collect();

        for new_achievement in &mut self.achievements {
            if let Some(existing) = existing_by_id.get(&new_achievement.id) {
                new_achievement.merge_metadata(existing);
            }
        }

        merge_achievements_with_existing(&mut self.achievements, &existing.achievements);
        merge_leaderboards_with_existing(&mut self.leaderboards, &existing.leaderboards);
    }
}

fn merge_achievements_with_existing(
    new_items: &mut Vec<AchievementEntry>,
    existing_items: &[AchievementEntry],
) {
    let new_ids: HashSet<_> = new_items
        .iter()
        .filter(|a| a.id > 0)
        .map(|a| a.id)
        .collect();
    let new_items_zero: HashSet<_> = new_items.iter().filter(|a| a.id == 0).cloned().collect();

    for existing in existing_items {
        let id = existing.id;
        if !(new_ids.contains(&id) || id == 0 && new_items_zero.contains(existing)) {
            new_items.push(existing.clone());
        }
    }
}

fn merge_leaderboards_with_existing(
    new_items: &mut Vec<LeaderboardEntry>,
    existing_items: &[LeaderboardEntry],
) {
    let new_ids: HashSet<_> = new_items
        .iter()
        .filter(|l| l.id > 0)
        .map(|l| l.id)
        .collect();
    let new_items_zero: HashSet<_> = new_items.iter().filter(|l| l.id == 0).cloned().collect();

    for existing in existing_items {
        let id = existing.id;
        if !(new_ids.contains(&id) || id == 0 && new_items_zero.contains(existing)) {
            new_items.push(existing.clone());
        }
    }
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

impl Display for UserFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
        let header = Header {
            version: PROTOCOL_VERSION.to_string(),
            game_title: set.game_name,
        };
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AchievementEntry {
    pub id: u32,
    pub condition: String,
    pub title: String,
    pub description: String,
    pub tag: Tag,
    pub author: Option<String>,
    pub points: u32,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub upvotes: Option<u32>,
    pub downvotes: Option<u32>,
    pub badge: Option<String>,
}

impl AchievementEntry {
    fn merge_metadata(&mut self, existing: &Self) {
        if self.author.is_none() {
            self.author = existing.author.clone();
        }
        if self.created.is_none() {
            self.created = existing.created.clone();
        }
        if self.updated.is_none() {
            self.updated = existing.updated.clone();
        }
        self.upvotes = self.upvotes.or(existing.upvotes);
        self.downvotes = self.downvotes.or(existing.downvotes);
        if self.badge.is_none() {
            self.badge = existing.badge.clone();
        }
    }
}

impl FromStr for AchievementEntry {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields = split_unquoted(s, ':');
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
        let author = fields
            .get(7)
            .map(|s| s.to_string())
            .filter(|v| !v.is_empty());
        let points = fields
            .get(8)
            .ok_or_else(|| ParseError::InvalidEntry("invalid points".into()))?
            .parse()
            .map_err(|_| ParseError::InvalidEntry("invalid points".into()))?;
        let created = fields
            .get(9)
            .map(|s| s.to_string())
            .filter(|v| !v.is_empty());
        let updated = fields
            .get(10)
            .map(|s| s.to_string())
            .filter(|v| !v.is_empty());
        let upvotes: Option<u32> = fields.get(11).and_then(|s| s.parse().ok());
        let downvotes: Option<u32> = fields.get(12).and_then(|s| s.parse().ok());
        let badge = fields
            .get(13)
            .map(|s| s.to_string())
            .filter(|v| !v.is_empty());

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

impl Display for AchievementEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:\"{}\":\"{}\":\"{}\": : :{}:{}:{}:{}:{}:{}:{}:{}",
            self.id,
            self.condition,
            self.title,
            self.description,
            self.tag,
            self.author.as_deref().unwrap_or(DEFAULT_AUTHOR),
            self.points,
            self.created.as_deref().unwrap_or(DEFAULT_TIMESTAMP),
            self.updated.as_deref().unwrap_or(DEFAULT_TIMESTAMP),
            self.upvotes.unwrap_or(DEFAULT_COUNT),
            self.downvotes.unwrap_or(DEFAULT_COUNT),
            self.badge.as_deref().unwrap_or(DEFAULT_BADGE),
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
            author: None,
            points,
            created: None,
            updated: None,
            upvotes: None,
            downvotes: None,
            badge: None,
        }
    }
}

impl From<Leaderboard> for LeaderboardEntry {
    fn from(leaderboard: Leaderboard) -> Self {
        let start = leaderboard.conditions.start.to_string();
        let cancel = leaderboard.conditions.cancel.to_string();
        let submit = leaderboard.conditions.submit.to_string();
        let value = leaderboard.conditions.value.to_string();
        let format = leaderboard.format.to_string();
        let title = leaderboard.title;
        let description = leaderboard.description;
        let id = leaderboard.id.unwrap_or(0);
        let lower_is_better = leaderboard.lower_is_better;

        LeaderboardEntry {
            id,
            start,
            cancel,
            submit,
            value,
            format,
            title,
            description,
            lower_is_better,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl FromStr for LeaderboardEntry {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields = split_unquoted(s, ':');
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

impl Display for LeaderboardEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

#[derive(Debug, Clone)]
pub struct CodeNote {
    pub address: u32,
    pub note: String,
}

impl FromStr for CodeNote {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix("N0:")
            .ok_or(ParseError::InvalidNote("missing N0: prefix".into()))?;
        let parts: Vec<_> = s.splitn(2, ':').collect();
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

impl Display for CodeNote {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "N0:0x{:x}:\"{}\"", self.address, self.note)
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
        assert_eq!(entry.author, Some("Author".to_string()));
        assert_eq!(entry.points, 3);
        assert_eq!(entry.upvotes, Some(0));
        assert_eq!(entry.downvotes, Some(0));
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
        let serialized = format!("{header}");
        let parsed: Header = serialized.parse().unwrap();
        assert_eq!(parsed.version, header.version);
        assert_eq!(parsed.game_title, header.game_title);
    }

    #[test]
    fn test_achievement_entry_roundtrip() {
        let input = "600707:\"I:0xH1a8c94*2_0xU1a9fad>=2\":\"Alpha Amateur \":\"Description\": : :progression:Author:3:0:0:0:0:00000";
        let entry: AchievementEntry = input.parse().unwrap();
        let serialized = format!("{entry}");
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
    fn test_achievement_entry_serialization_defaults() {
        let entry = AchievementEntry {
            id: 123,
            condition: "0xH0001".into(),
            title: "Test".into(),
            description: "Test desc".into(),
            tag: Tag::Progression,
            author: None,
            points: 5,
            created: None,
            updated: None,
            upvotes: None,
            downvotes: None,
            badge: None,
        };

        let serialized = entry.to_string();
        let parsed: AchievementEntry = serialized.parse().unwrap();

        assert_eq!(parsed.created, Some("0".to_string()));
        assert_eq!(parsed.updated, Some("0".to_string()));
        assert_eq!(parsed.upvotes, Some(0));
        assert_eq!(parsed.downvotes, Some(0));
        assert_eq!(parsed.badge, Some("00000".to_string()));
    }

    #[test]
    fn test_leaderboard_entry_roundtrip() {
        let input = "L0:\"start\":\"cancel\":\"submit\":\"M:0xX31d048\":VALUE:\"Title\":\"Desc\":0";
        let entry: LeaderboardEntry = input.parse().unwrap();
        let serialized = format!("{entry}");
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
        let serialized = format!("{note}");
        let parsed: CodeNote = serialized.parse().unwrap();
        assert_eq!(parsed.address, note.address);
        assert_eq!(parsed.note, note.note);
    }

    #[test]
    fn test_user_file_roundtrip() {
        let input = "1.3\nGame Title\n600707:\"cond\":\"Title\":\"Desc\": : :progression:Author:3:0:0:0:0:00000\nL0:\"s\":\"c\":\"sub\":\"v\":VALUE:\"T\":\"D\":0\nN0:0x123:\"Note\"";
        let file: UserFile = input.parse().unwrap();
        let serialized = format!("{file}");
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
        set.add(achievement);
        let user_file = UserFile::from(set);
        let serialized = format!("{user_file}");
        let parsed: UserFile = serialized.parse().unwrap();
        assert_eq!(parsed.header.version, "1.3");
        assert_eq!(parsed.header.game_title, "Test Game");
        assert_eq!(parsed.achievements.len(), 1);
        assert_eq!(parsed.achievements[0].title, "Test Achievement");
        assert_eq!(parsed.achievements[0].points, 5);
    }

    #[test]
    fn test_merge_with_existing() {
        let mut new_file = UserFile {
            header: Header {
                version: "1.3".into(),
                game_title: "Test Game".into(),
            },
            achievements: vec![AchievementEntry {
                id: 1,
                condition: "cond1".into(),
                title: "Achievement 1".into(),
                description: "Desc".into(),
                tag: Tag::Progression,
                author: None,
                points: 5,
                created: None,
                updated: None,
                upvotes: None,
                downvotes: None,
                badge: None,
            }],
            leaderboards: Vec::new(),
            notes: Vec::new(),
        };

        let existing = UserFile {
            header: Header {
                version: "1.3".into(),
                game_title: "Test Game".into(),
            },
            achievements: vec![AchievementEntry {
                id: 1,
                condition: "cond1".into(),
                title: "Achievement 1".into(),
                description: "Desc".into(),
                tag: Tag::Progression,
                author: Some("ExistingAuthor".into()),
                points: 5,
                created: Some("1234567890".into()),
                updated: Some("1234567891".into()),
                upvotes: Some(10),
                downvotes: Some(2),
                badge: Some("12345".into()),
            }],
            leaderboards: vec![LeaderboardEntry {
                id: 0,
                start: "start".into(),
                cancel: "cancel".into(),
                submit: "submit".into(),
                value: "value".into(),
                format: "VALUE".into(),
                title: "Leaderboard".into(),
                description: "Desc".into(),
                lower_is_better: false,
            }],
            notes: vec![CodeNote {
                address: 0x1000,
                note: "Test note".into(),
            }],
        };

        new_file.merge_with_existing(&existing);

        assert_eq!(
            new_file.achievements[0].author,
            Some("ExistingAuthor".to_string())
        );
        assert_eq!(
            new_file.achievements[0].created,
            Some("1234567890".to_string())
        );
        assert_eq!(
            new_file.achievements[0].updated,
            Some("1234567891".to_string())
        );
        assert_eq!(new_file.achievements[0].upvotes, Some(10));
        assert_eq!(new_file.achievements[0].downvotes, Some(2));
        assert_eq!(new_file.achievements[0].badge, Some("12345".to_string()));
    }

    #[test]
    fn test_merge_preserves_existing_achievements() {
        let mut new_file = UserFile {
            header: Header {
                version: "1.3".into(),
                game_title: "Test".into(),
            },
            achievements: vec![AchievementEntry {
                id: 1,
                condition: "cond".into(),
                title: "Ach1".into(),
                description: "Desc".into(),
                tag: Tag::Progression,
                author: None,
                points: 5,
                created: None,
                updated: None,
                upvotes: None,
                downvotes: None,
                badge: None,
            }],
            leaderboards: Vec::new(),
            notes: Vec::new(),
        };

        let existing = UserFile {
            header: Header {
                version: "1.3".into(),
                game_title: "Test".into(),
            },
            achievements: vec![
                AchievementEntry {
                    id: 1,
                    condition: "cond".into(),
                    title: "Ach1".into(),
                    description: "Desc".into(),
                    tag: Tag::Progression,
                    author: Some("Author1".into()),
                    points: 5,
                    created: Some("1".into()),
                    updated: Some("1".into()),
                    upvotes: Some(5),
                    downvotes: Some(1),
                    badge: Some("1".into()),
                },
                AchievementEntry {
                    id: 2,
                    condition: "cond2".into(),
                    title: "Ach2".into(),
                    description: "Desc2".into(),
                    tag: Tag::WinCondition,
                    author: Some("Author2".into()),
                    points: 10,
                    created: Some("2".into()),
                    updated: Some("2".into()),
                    upvotes: Some(20),
                    downvotes: Some(5),
                    badge: Some("2".into()),
                },
            ],
            leaderboards: Vec::new(),
            notes: Vec::new(),
        };

        new_file.merge_with_existing(&existing);

        assert_eq!(new_file.achievements.len(), 2);
        assert_eq!(new_file.achievements[0].id, 1);
        assert_eq!(new_file.achievements[0].author, Some("Author1".to_string()));
        assert_eq!(new_file.achievements[1].id, 2);
        assert_eq!(new_file.achievements[1].author, Some("Author2".to_string()));
    }
}

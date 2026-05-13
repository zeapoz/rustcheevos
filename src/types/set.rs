use std::{fs, io, path::Path};

use crate::schema::user::{USER_FILE_EXTENSION, USER_FILE_SUFFIX, UserFile};

use super::{achievement::Achievement, leaderboard::Leaderboard};

/// An achievement set containing achievements and leaderboards.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Set {
    /// The game ID.
    pub game_id: String,
    /// The game name.
    pub game_name: String,
    /// The achievements in this set.
    pub achievements: Vec<Achievement>,
    /// The leaderboards in this set.
    pub leaderboards: Vec<Leaderboard>,
}

impl Set {
    /// Creates a new achievement set with the given game ID and name.
    ///
    /// # Arguments
    ///
    /// * `game_id` - The game ID.
    /// * `game_name` - The game name.
    pub fn new(game_id: impl Into<String>, game_name: impl Into<String>) -> Self {
        Self {
            game_id: game_id.into(),
            game_name: game_name.into(),
            achievements: Vec::new(),
            leaderboards: Vec::new(),
        }
    }

    /// Adds an achievement or leaderboard to this set.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to add ([`Achievement`] or [`Leaderboard`]).
    pub fn add(&mut self, item: impl Into<SetItem>) -> &mut Self {
        match item.into() {
            SetItem::Achievement(achievement) => self.achievements.push(achievement),
            SetItem::Leaderboard(leaderboard) => self.leaderboards.push(leaderboard),
        }
        self
    }

    /// Adds multiple items to this set.
    ///
    /// # Arguments
    ///
    /// * `items` - The items to add.
    pub fn add_many(&mut self, items: impl IntoIterator<Item = impl Into<SetItem>>) -> &mut Self {
        for item in items {
            self.add(item);
        }
        self
    }

    /// Exports this set to to the user file at the given directory.
    ///
    /// # Arguments
    ///
    /// * `dir` - The directory to export to.
    pub fn export(&self, dir: impl AsRef<Path>) -> io::Result<()> {
        let filename = format!(
            "{}{}.{}",
            self.game_id, USER_FILE_SUFFIX, USER_FILE_EXTENSION
        );
        let path = dir.as_ref().join(filename);
        self.export_to_file(path)
    }

    /// Exports this set to a custom file path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to export to.
    pub fn export_to_file(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let user_file = UserFile::from(self.clone());
        fs::write(path, user_file.to_string())
    }
}

/// Items that can be part of an achievement set.
#[derive(Debug, Clone, PartialEq)]
pub enum SetItem {
    /// An achievement.
    Achievement(Achievement),
    /// A leaderboard.
    Leaderboard(Leaderboard),
}

impl From<Achievement> for SetItem {
    fn from(achievement: Achievement) -> Self {
        SetItem::Achievement(achievement)
    }
}

impl From<Leaderboard> for SetItem {
    fn from(leaderboard: Leaderboard) -> Self {
        SetItem::Leaderboard(leaderboard)
    }
}

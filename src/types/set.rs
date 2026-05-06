//! Achievement set types and serialization.

use std::path::Path;

use thiserror::Error;

use super::achievement::Achievement;
use super::leaderboard::Leaderboard;
use crate::schema::user::UserFile;

/// Errors that can occur when exporting an achievement set.
#[derive(Error, Debug)]
pub enum ExportError {
    /// I/O error during export.
    #[error("export failed: {0}")]
    Io(#[from] std::io::Error),
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
    pub fn new<S: Into<String>>(game_id: S, game_name: S) -> Self {
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
    pub fn add_many(&mut self, items: impl IntoIterator<Item = SetItem>) -> &mut Self {
        for item in items {
            self.add(item);
        }
        self
    }

    /// Exports this set to a file in the given directory.
    ///
    /// # Arguments
    ///
    /// * `dir` - The directory to export to.
    ///
    /// # Errors
    /// Returns [`ExportError`] if writing fails.
    pub fn export(&self, dir: impl AsRef<Path>) -> Result<(), ExportError> {
        let filename = format!("{}-User.txt", self.game_id);
        let path = dir.as_ref().join(filename);

        let mut new_user_file = UserFile::from(self.clone());

        if let Ok(existing_content) = std::fs::read_to_string(&path) {
            if let Ok(existing) = existing_content.parse::<UserFile>() {
                new_user_file.merge_with_existing(&existing);
                new_user_file.notes = existing.notes;
            }
        }

        Ok(std::fs::write(path, new_user_file.to_string())?)
    }
}

use std::path::Path;

use thiserror::Error;

use super::achievement::Achievement;
use super::leaderboard::Leaderboard;
use crate::schema::user::UserFile;

#[derive(Error, Debug)]
pub enum ExportError {
    #[error("export failed: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SetItem {
    Achievement(Achievement),
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

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Set {
    pub game_id: String,
    pub game_name: String,
    pub achievements: Vec<Achievement>,
    pub leaderboards: Vec<Leaderboard>,
}

impl Set {
    pub fn new<S: Into<String>>(game_id: S, game_name: S) -> Self {
        Self {
            game_id: game_id.into(),
            game_name: game_name.into(),
            achievements: Vec::new(),
            leaderboards: Vec::new(),
        }
    }

    pub fn add(&mut self, item: impl Into<SetItem>) -> &mut Self {
        match item.into() {
            SetItem::Achievement(achievement) => self.achievements.push(achievement),
            SetItem::Leaderboard(leaderboard) => self.leaderboards.push(leaderboard),
        }
        self
    }

    pub fn add_many(&mut self, items: impl IntoIterator<Item = SetItem>) -> &mut Self {
        for item in items {
            self.add(item);
        }
        self
    }

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

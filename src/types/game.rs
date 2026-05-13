use std::{fs, io, path::Path};

use crate::{
    prelude::{Achievement, Leaderboard},
    schema::user::{USER_FILE_EXTENSION, USER_FILE_SUFFIX, UserFile},
};

use super::rich::RichPresence;

pub type AchievementSet = Vec<Achievement>;
pub type LeaderboardSet = Vec<Leaderboard>;

#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    /// The game ID.
    id: String,
    /// The game name.
    title: String,
    /// The core achievement set.
    core_set: AchievementSet,
    /// The leaderboards.
    leaderboards: LeaderboardSet,
    /// The rich presence.
    rich_presence: RichPresence,
}

impl Game {
    /// Creates a new game.
    ///
    /// # Arguments
    ///
    /// * `id` - The game ID.
    /// * `name` - The game name.
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: name.into(),
            core_set: AchievementSet::new(),
            leaderboards: LeaderboardSet::new(),
            rich_presence: RichPresence::new(),
        }
    }

    /// Adds an asset to this game.
    ///
    /// # Arguments
    ///
    /// * `item` - The asset to add.
    pub fn add(&mut self, item: impl Into<GameAsset>) -> &mut Self {
        match item.into() {
            GameAsset::Achievement(achievement) => self.core_set.push(achievement),
            GameAsset::Leaderboard(leaderboard) => self.leaderboards.push(leaderboard),
            GameAsset::RichPresence(rich_presence) => self.rich_presence = rich_presence,
        };
        self
    }

    /// Adds multiple assets to this game.
    ///
    /// # Arguments
    ///
    /// * `items` - The assets to add.
    pub fn add_many(&mut self, items: impl IntoIterator<Item = impl Into<GameAsset>>) -> &mut Self {
        for item in items {
            self.add(item);
        }
        self
    }

    /// Sets the core achievement set for this game.
    ///
    /// # Arguments
    ///
    /// * `core_set` - The core achievement set to set.
    pub fn set_core_set(&mut self, core_set: impl Into<AchievementSet>) -> &mut Self {
        self.core_set = core_set.into();
        self
    }

    /// Sets the leaderboards for this game.
    ///
    /// # Arguments
    ///
    /// * `leaderboards` - The leaderboards to set.
    pub fn set_leaderboards(&mut self, leaderboards: impl Into<LeaderboardSet>) -> &mut Self {
        self.leaderboards = leaderboards.into();
        self
    }

    /// Sets the rich presence for this game.
    ///
    /// # Arguments
    ///
    /// * `rich_presence` - The rich presence to set.
    pub fn set_rich_presence(&mut self, rich_presence: impl Into<RichPresence>) -> &mut Self {
        self.rich_presence = rich_presence.into();
        self
    }

    /// Returns an iterator over the achievements in this game.
    pub fn achievements(&self) -> impl Iterator<Item = &Achievement> {
        self.core_set.iter()
    }

    /// Returns an iterator over the leaderboards in this game.
    pub fn leaderboards(&self) -> impl Iterator<Item = &Leaderboard> {
        self.leaderboards.iter()
    }

    /// Returns the user file representation of this game.
    fn user_file(&self) -> UserFile {
        UserFile::new(self.title.clone(), self.achievements(), self.leaderboards())
    }

    /// Exports the assets of this game to the given directory.
    ///
    /// # Arguments
    ///
    /// * `dir` - The directory to export to.
    pub fn export(&self, dir: impl AsRef<Path>) -> io::Result<()> {
        let dir = dir.as_ref();
        self.export_user_file(dir)?;
        self.rich_presence.export(&self.id, dir)
    }

    /// Exports the user file for this game to the given directory.
    ///
    /// # Arguments
    ///
    /// * `dir` - The directory to export to.
    pub fn export_user_file(&self, dir: impl AsRef<Path>) -> io::Result<()> {
        let user_file = self.user_file();
        let filename = format!("{}{USER_FILE_SUFFIX}{USER_FILE_EXTENSION}", self.id);
        let path = dir.as_ref().join(filename);
        fs::write(path, user_file.to_string())
    }
}

/// An asset for a game.
#[derive(Debug, Clone, PartialEq)]
pub enum GameAsset {
    Achievement(Achievement),
    Leaderboard(Leaderboard),
    RichPresence(RichPresence),
}

impl From<Achievement> for GameAsset {
    fn from(achievement: Achievement) -> Self {
        GameAsset::Achievement(achievement)
    }
}

impl From<Leaderboard> for GameAsset {
    fn from(leaderboard: Leaderboard) -> Self {
        GameAsset::Leaderboard(leaderboard)
    }
}

impl From<RichPresence> for GameAsset {
    fn from(rich_presence: RichPresence) -> Self {
        GameAsset::RichPresence(rich_presence)
    }
}

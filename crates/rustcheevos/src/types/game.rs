//! Type definition for the core game container struct.

use std::{fs, io, path::Path};

use crate::{
    prelude::{Achievement, CodeNote, Leaderboard},
    schema::user::{USER_FILE_EXTENSION, USER_FILE_SUFFIX, UserFile},
};

use super::rich::RichPresence;

/// A set of achievements.
pub type AchievementSet = Vec<Achievement>;
/// A set of leaderboards.
pub type LeaderboardSet = Vec<Leaderboard>;
/// A set of code notes.
pub type CodeNoteSet = Vec<CodeNote>;

/// The core game struct containing all the assets.
///
/// # Examples
///
/// ```no_run
/// use rustcheevos::{prelude::*, bits8, chain, measured};
///
/// // Create a new game.
/// let mut game_data = GameData::new(123, "Super Adventure");
///
/// // Define an achievement with conditions.
/// let achievement_condition = chain!(
///     bits8!(0x1234).eq(1),
///     bits8!(0x5678).ge(100),
/// );
/// let achievement = Achievement::new(
///     "First Step",
///     "Complete the tutorial level",
///     achievement_condition,
///     5,
/// );
///
/// // Define a leaderboard with conditions.
/// let start = chain!(bits8!(0x1234).eq(1));
/// let cancel = chain!(bits8!(0x1234).eq(0));
/// let submit = chain!(bits8!(0xABCD).eq(1));
/// let value = measured!(bits8!(0xDEF0));
/// let leaderboard = Leaderboard::new(
///     "Speed Run",
///     "Complete the game as fast as possible",
///     start,
///     cancel,
///     submit,
///     value,
///     LeaderboardFormat::Seconds,
///     true,
/// );
///
/// // Define rich presence.
/// let mut rich_presence = RichPresence::new();
/// let display_condition = chain!(bits8!(0x1234).ge(1));
/// rich_presence.add_conditional_display(display_condition, "Playing: @Stage(0x1234)");
/// rich_presence.add_static_display("Super Adventure");
///
/// // Add all assets to the game.
/// game_data.add(achievement)
///     .add(leaderboard)
///     .set_rich_presence(rich_presence);
///
/// // Export to a directory.
/// let directory = std::env::temp_dir().join("rustcheevos_example");
/// std::fs::create_dir_all(&directory).unwrap();
/// game_data.export(&directory).unwrap();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct GameData {
    /// The game ID.
    id: u32,
    /// The game name.
    title: String,
    /// The core achievement set.
    core_set: AchievementSet,
    /// The leaderboards.
    leaderboards: LeaderboardSet,
    /// The code notes.
    code_notes: CodeNoteSet,
    /// The rich presence.
    rich_presence: RichPresence,
}

impl GameData {
    /// Creates a new game with the given ID and name.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let game_data = GameData::new(1, "Super Adventure");
    /// ```
    pub fn new(id: u32, name: impl Into<String>) -> Self {
        Self {
            id,
            title: name.into(),
            core_set: AchievementSet::new(),
            leaderboards: LeaderboardSet::new(),
            code_notes: CodeNoteSet::new(),
            rich_presence: RichPresence::new(),
        }
    }

    /// Adds an asset to this game.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustcheevos::{prelude::*, chain, bits8};
    /// let mut game_data = GameData::new(1, "Super Adventure");
    ///
    /// let condition = chain!(bits8!(0x1234).eq(1));
    /// let achievement = Achievement::new("First Step", "Complete the tutorial", condition, 5);
    ///
    /// game_data.add(achievement);
    /// assert_eq!(game_data.achievements().count(), 1);
    /// ```
    pub fn add(&mut self, item: impl Into<GameAsset>) -> &mut Self {
        match item.into() {
            GameAsset::Achievement(achievement) => self.core_set.push(achievement),
            GameAsset::Leaderboard(leaderboard) => self.leaderboards.push(leaderboard),
            GameAsset::CodeNote(note) => self.code_notes.push(note),
            GameAsset::RichPresence(rich_presence) => self.rich_presence = rich_presence,
        }
        self
    }

    /// Adds multiple assets to this game.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustcheevos::{prelude::*, chain, bits8};
    /// let mut game_data = GameData::new(1, "Super Adventure");
    ///
    /// let condition = chain!(bits8!(0x1234).eq(1));
    /// let achievement_a = Achievement::new("Step A", "Do A", condition.clone(), 5);
    /// let achievement_b = Achievement::new("Step B", "Do B", condition, 10);
    ///
    /// game_data.add_many([achievement_a, achievement_b]);
    /// assert_eq!(game_data.achievements().count(), 2);
    /// ```
    pub fn add_many(&mut self, items: impl IntoIterator<Item = impl Into<GameAsset>>) -> &mut Self {
        for item in items {
            self.add(item);
        }
        self
    }

    /// Sets the core achievement set for this game.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustcheevos::{prelude::*, chain, bits8};
    /// let mut game_data = GameData::new(1, "Super Adventure");
    ///
    /// let condition = chain!(bits8!(0x1234).eq(1));
    /// let achievement = Achievement::new("First Step", "Complete the tutorial", condition, 5);
    ///
    /// game_data.set_core_set(vec![achievement]);
    /// assert_eq!(game_data.achievements().count(), 1);
    /// ```
    pub fn set_core_set(&mut self, core_set: impl Into<AchievementSet>) -> &mut Self {
        self.core_set = core_set.into();
        self
    }

    /// Sets the leaderboards for this game.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustcheevos::{prelude::*, chain, bits8, measured};
    /// let mut game_data = GameData::new(1, "Super Adventure");
    ///
    /// let start = chain!(bits8!(0x1234).eq(1));
    /// let cancel = chain!(bits8!(0x1234).eq(0));
    /// let submit = chain!(bits8!(0xABCD).eq(1));
    /// let value = measured!(bits8!(0xDEF0));
    /// let leaderboard = Leaderboard::new(
    ///     "Speed Run",
    ///     "Complete the game fast",
    ///     start,
    ///     cancel,
    ///     submit,
    ///     value,
    ///     LeaderboardFormat::Seconds,
    ///     true,
    /// );
    ///
    /// game_data.set_leaderboards(vec![leaderboard]);
    /// assert_eq!(game_data.leaderboards().count(), 1);
    /// ```
    pub fn set_leaderboards(&mut self, leaderboards: impl Into<LeaderboardSet>) -> &mut Self {
        self.leaderboards = leaderboards.into();
        self
    }

    /// Sets the code notes for this game.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let mut game_data = GameData::new(1, "Super Adventure");
    ///
    /// let note = CodeNote::new(0x1234, "Player health");
    /// game_data.set_code_notes(vec![note]);
    /// assert_eq!(game_data.code_notes().count(), 1);
    /// ```
    pub fn set_code_notes(&mut self, code_notes: impl Into<CodeNoteSet>) -> &mut Self {
        self.code_notes = code_notes.into();
        self
    }

    /// Sets the rich presence for this game.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustcheevos::prelude::*;
    /// let mut game_data = GameData::new(1, "Super Adventure");
    ///
    /// let mut rich_presence = RichPresence::new();
    /// rich_presence.add_static_display("Playing Super Adventure");
    ///
    /// game_data.set_rich_presence(rich_presence);
    /// ```
    pub fn set_rich_presence(&mut self, rich_presence: impl Into<RichPresence>) -> &mut Self {
        self.rich_presence = rich_presence.into();
        self
    }

    /// Returns an iterator over the achievements in this game.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustcheevos::prelude::*;
    /// let game_data = GameData::new(1, "Test");
    ///
    /// for achievement in game_data.achievements() {
    ///     println!("{}", achievement.title);
    /// }
    /// ```
    pub fn achievements(&self) -> impl Iterator<Item = &Achievement> {
        self.core_set.iter()
    }

    /// Returns an iterator over the leaderboards in this game.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustcheevos::prelude::*;
    /// let game_data = GameData::new(1, "Test");
    ///
    /// for lb in game_data.leaderboards() {
    ///     println!("{}", lb.title);
    /// }
    /// ```
    pub fn leaderboards(&self) -> impl Iterator<Item = &Leaderboard> {
        self.leaderboards.iter()
    }

    /// Returns an iterator over the code notes in this game.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let game_data = GameData::new(1, "Test");
    ///
    /// for note in game_data.code_notes() {
    ///     println!("{:x}: {}", note.address, note.contents);
    /// }
    /// ```
    pub fn code_notes(&self) -> impl Iterator<Item = &CodeNote> {
        self.code_notes.iter()
    }

    /// Returns the user file representation of this game.
    fn user_file(&self) -> UserFile {
        UserFile::new(
            self.title.clone(),
            self.achievements(),
            self.leaderboards(),
            self.code_notes(),
        )
    }

    /// Exports the assets of this game to the given directory.
    ///
    /// This exports the user file and the rich presence file to the given directory.
    ///
    /// # Errors
    ///
    /// Returns an error if the directory cannot be created or if writing fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustcheevos::prelude::*;
    ///
    /// let mut game_data = GameData::new(1, "Super Adventure");
    ///
    /// let mut rich_presence = RichPresence::new();
    /// rich_presence.add_static_display("Playing Super Adventure");
    /// game_data.set_rich_presence(rich_presence);
    ///
    /// let temp_dir = std::env::temp_dir().join("rustcheevos_export_test");
    /// std::fs::create_dir_all(&temp_dir).unwrap();
    ///
    /// game_data.export(&temp_dir).unwrap();
    /// ```
    pub fn export(&self, dir: impl AsRef<Path>) -> io::Result<()> {
        let dir = dir.as_ref();
        self.export_user_file(dir)?;
        self.rich_presence.export(self.id, dir)
    }

    /// Exports the user file for this game to the given directory.
    ///
    /// # Errors
    ///
    /// Returns an error if the directory cannot be created or if writing fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustcheevos::{prelude::*, chain, bits8};
    ///
    /// let mut game_data = GameData::new(1, "Super Adventure");
    ///
    /// let condition = chain!(bits8!(0x1234).eq(1));
    /// let achievement = Achievement::new("First Step", "Complete the tutorial", condition, 5);
    /// game_data.add(achievement);
    ///
    /// let temp_dir = std::env::temp_dir().join("rustcheevos_user_file_test");
    /// std::fs::create_dir_all(&temp_dir).unwrap();
    ///
    /// game_data.export_user_file(&temp_dir).unwrap();
    /// ```
    pub fn export_user_file(&self, dir: impl AsRef<Path>) -> io::Result<()> {
        let user_file = self.user_file();
        let filename = format!("{}{USER_FILE_SUFFIX}.{USER_FILE_EXTENSION}", self.id);
        let path = dir.as_ref().join(filename);
        fs::write(path, user_file.to_string())
    }
}

/// An asset for a game.
///
/// # Examples
///
/// ```
/// use rustcheevos::{prelude::*, chain, bits8};
/// use rustcheevos::types::game::GameAsset;
///
/// // GameAsset can be created from Achievement, Leaderboard, or RichPresence
/// let condition = chain!(bits8!(0x1234).eq(1));
/// let achievement = Achievement::new("First Step", "Complete the tutorial", condition, 5);
///
/// let game_asset: GameAsset = achievement.into();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum GameAsset {
    /// An achievement.
    Achievement(Achievement),
    /// A leaderboard.
    Leaderboard(Leaderboard),
    /// A code note.
    CodeNote(CodeNote),
    /// A rich presence.
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

impl From<CodeNote> for GameAsset {
    fn from(note: CodeNote) -> Self {
        GameAsset::CodeNote(note)
    }
}

impl From<RichPresence> for GameAsset {
    fn from(rich_presence: RichPresence) -> Self {
        GameAsset::RichPresence(rich_presence)
    }
}

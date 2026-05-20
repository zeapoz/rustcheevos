//! Type definition for the core game container struct.

use crate::types::{
    achievement::Achievement, leaderboard::Leaderboard, note::CodeNote, rich::RichPresence,
};
use rustcheevos_schema::user::{AchievementEntry, CodeNoteEntry, LeaderboardEntry, UserFile};

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
/// use rustcheevos::prelude::*;
/// use rustcheevos::types::{
///     achievement::Achievement,
///     game::GameData,
///     leaderboard::{Leaderboard, LeaderboardFormat},
///     note::CodeNote,
///     rich::{Entry, LookupTable, RichPresence},
/// };
/// use rustcheevos::{bits8, chain, measured};
///
/// // Create a new game.
/// let mut game_data = GameData::new(123, "Super Adventure");
///
/// // Define an achievement with conditions.
/// let achievement = Achievement::builder("First Step")
///     .description("Complete the tutorial level")
///     .requirements(chain!(
///         bits8!(0x1234).eq(1),
///         bits8!(0x5678).ge(100),
///     ))
///     .badge_id(12345)
///     .points(5)
///     .build();
///
/// // Define a leaderboard with conditions.
/// let leaderboard = Leaderboard::builder("Speed Run")
///     .description("Complete the game as fast as possible")
///     .start(chain!(bits8!(0x1234).eq(1)))
///     .cancel(chain!(bits8!(0x1234).eq(0)))
///     .submit(chain!(bits8!(0xABCD).eq(1)))
///     .value(measured!(bits8!(0xDEF0)))
///     .format(LeaderboardFormat::Seconds)
///     .lower_is_better(true)
///     .build();
///
/// // Define a code note.
/// let note = CodeNote::new(0x1234, "Player health");
///
/// // Define rich presence.
/// let mut rich_presence = RichPresence::new();
/// let table = LookupTable::new("Stage")
///     .with_entry(Entry::new(1, "Level 1"))
///     .with_fallback("Main Menu");
/// let stage = rich_presence.register_lookup(table, bits8!(0x1234));
/// let display_condition = chain!(bits8!(0x1234).ge(1));
/// rich_presence.add_conditional_display(display_condition, format!("Playing: {stage}"));
/// rich_presence.add_static_display("Super Adventure");
///
/// // Add all assets to the game.
/// game_data
///     .add(achievement)
///     .add(leaderboard)
///     .add(note)
///     .set_rich_presence(rich_presence);
///
/// // Serialize to the user file format.
/// let user_file = game_data.to_user_file();
/// println!("{user_file}");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct GameData {
    /// The game ID.
    id: u32,
    /// The game name.
    title: String,
    /// The achievements.
    achievements: AchievementSet,
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
    /// use rustcheevos::types::game::GameData;
    ///
    /// let game_data = GameData::new(1, "Super Adventure");
    /// ```
    #[must_use]
    pub fn new(id: u32, name: impl Into<String>) -> Self {
        Self {
            id,
            title: name.into(),
            achievements: AchievementSet::new(),
            leaderboards: LeaderboardSet::new(),
            code_notes: CodeNoteSet::new(),
            rich_presence: RichPresence::new(),
        }
    }

    /// Returns the game ID.
    #[must_use]
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns the game title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Adds an asset to this game.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustcheevos::prelude::*;
    /// # use rustcheevos::types::{achievement::Achievement, game::GameData};
    /// # use rustcheevos::{chain, bits8};
    /// let mut game_data = GameData::new(1, "Super Adventure");
    ///
    /// let condition = chain!(bits8!(0x1234).eq(1));
    /// let achievement = Achievement::builder("First Step")
    ///     .description("Complete the tutorial")
    ///     .requirements(condition)
    ///     .badge_id(12345)
    ///     .points(5)
    ///     .build();
    ///
    /// game_data.add(achievement);
    /// assert_eq!(game_data.achievements().len(), 1);
    /// ```
    pub fn add(&mut self, item: impl Into<GameAsset>) -> &mut Self {
        match item.into() {
            GameAsset::Achievement(achievement) => self.achievements.push(achievement),
            GameAsset::Leaderboard(leaderboard) => self.leaderboards.push(leaderboard),
            GameAsset::CodeNote(note) => self.code_notes.push(note),
        }
        self
    }

    /// Adds multiple assets to this game.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustcheevos::prelude::*;
    /// # use rustcheevos::types::{achievement::Achievement, game::GameData};
    /// # use rustcheevos::{chain, bits8};
    /// let mut game_data = GameData::new(1, "Super Adventure");
    ///
    /// let condition = chain!(bits8!(0x1234).eq(1));
    /// let achievement_a = Achievement::builder("Step A")
    ///     .description("Do A")
    ///     .requirements(condition.clone())
    ///     .badge_id(12345)
    ///     .points(5)
    ///     .build();
    /// let achievement_b = Achievement::builder("Step B")
    ///     .description("Do B")
    ///     .requirements(condition)
    ///     .badge_id(12345)
    ///     .points(10)
    ///     .build();
    ///
    /// game_data.add_many([achievement_a, achievement_b]);
    /// assert_eq!(game_data.achievements().len(), 2);
    /// ```
    pub fn add_many(&mut self, items: impl IntoIterator<Item = impl Into<GameAsset>>) -> &mut Self {
        for item in items {
            self.add(item);
        }
        self
    }

    /// Sets the achievements for this game.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustcheevos::prelude::*;
    /// # use rustcheevos::types::{achievement::Achievement, game::GameData};
    /// # use rustcheevos::{chain, bits8};
    /// let mut game_data = GameData::new(1, "Super Adventure");
    ///
    /// let achievement = Achievement::builder("First Step")
    ///     .description("Complete the tutorial")
    ///     .requirements(chain!(bits8!(0x1234).eq(1)))
    ///     .badge_id(12345)
    ///     .points(5)
    ///     .build();
    ///
    /// game_data.set_achievements(vec![achievement]);
    /// assert_eq!(game_data.achievements().len(), 1);
    /// ```
    pub fn set_achievements(&mut self, achievements: impl Into<AchievementSet>) -> &mut Self {
        self.achievements = achievements.into();
        self
    }

    /// Sets the leaderboards for this game.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustcheevos::prelude::*;
    /// # use rustcheevos::types::{game::GameData, leaderboard::{Leaderboard, LeaderboardFormat}};
    /// # use rustcheevos::{chain, bits8, measured};
    /// let mut game_data = GameData::new(1, "Super Adventure");
    ///
    /// let leaderboard = Leaderboard::builder("Speed Run")
    ///     .description("Complete the game fast")
    ///     .start(chain!(bits8!(0x1234).eq(1)))
    ///     .cancel(chain!(bits8!(0x1234).eq(0)))
    ///     .submit(chain!(bits8!(0xABCD).eq(1)))
    ///     .value(measured!(bits8!(0xDEF0)))
    ///     .format(LeaderboardFormat::Seconds)
    ///     .lower_is_better(true)
    ///     .build();
    ///
    /// game_data.set_leaderboards(vec![leaderboard]);
    /// assert_eq!(game_data.leaderboards().len(), 1);
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
    /// use rustcheevos::types::{game::GameData, note::CodeNote};
    ///
    /// let mut game_data = GameData::new(1, "Super Adventure");
    ///
    /// let note = CodeNote::new(0x1234, "Player health");
    /// game_data.set_code_notes(vec![note]);
    /// assert_eq!(game_data.code_notes().len(), 1);
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
    /// # use rustcheevos::types::{game::GameData, rich::RichPresence};
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

    /// Returns the achievements for this game.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustcheevos::prelude::*;
    /// # use rustcheevos::types::game::GameData;
    /// let game_data = GameData::new(1, "Test");
    ///
    /// assert_eq!(game_data.achievements().len(), 0);
    /// ```
    #[must_use]
    pub fn achievements(&self) -> &[Achievement] {
        &self.achievements
    }

    /// Returns the leaderboards for this game.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustcheevos::prelude::*;
    /// # use rustcheevos::types::game::GameData;
    /// let game_data = GameData::new(1, "Test");
    ///
    /// assert_eq!(game_data.leaderboards().len(), 0);
    /// ```
    #[must_use]
    pub fn leaderboards(&self) -> &[Leaderboard] {
        &self.leaderboards
    }

    /// Returns the code notes for this game.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::game::GameData;
    ///
    /// let game_data = GameData::new(1, "Test");
    ///
    /// assert_eq!(game_data.code_notes().len(), 0);
    /// ```
    #[must_use]
    pub fn code_notes(&self) -> &[CodeNote] {
        &self.code_notes
    }

    /// Returns an iterator over the achievements in this game.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustcheevos::prelude::*;
    /// # use rustcheevos::types::game::GameData;
    /// let game_data = GameData::new(1, "Test");
    ///
    /// for achievement in game_data.iter_achievements() {
    ///     println!("{}", achievement.title());
    /// }
    /// ```
    pub fn iter_achievements(&self) -> impl Iterator<Item = &Achievement> {
        self.achievements.iter()
    }

    /// Returns an iterator over the leaderboards in this game.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustcheevos::prelude::*;
    /// # use rustcheevos::types::game::GameData;
    /// let game_data = GameData::new(1, "Test");
    ///
    /// for lb in game_data.iter_leaderboards() {
    ///     println!("{}", lb.title());
    /// }
    /// ```
    pub fn iter_leaderboards(&self) -> impl Iterator<Item = &Leaderboard> {
        self.leaderboards.iter()
    }

    /// Returns an iterator over the code notes in this game.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::game::GameData;
    ///
    /// let game_data = GameData::new(1, "Test");
    ///
    /// for note in game_data.iter_code_notes() {
    ///     println!("{:x}: {}", note.address(), note.contents());
    /// }
    /// ```
    pub fn iter_code_notes(&self) -> impl Iterator<Item = &CodeNote> {
        self.code_notes.iter()
    }

    /// Returns the rich presence for this game.
    #[must_use]
    pub fn rich_presence(&self) -> &RichPresence {
        &self.rich_presence
    }

    /// Returns the user file representation of this game.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustcheevos::prelude::*;
    /// # use rustcheevos::types::{achievement::Achievement, game::GameData};
    /// # use rustcheevos::{chain, bits8};
    /// let mut game_data = GameData::new(1, "Super Adventure");
    ///
    /// let condition = chain!(bits8!(0x1234).eq(1));
    /// let achievement = Achievement::builder("First Step")
    ///     .description("Complete the tutorial")
    ///     .requirements(condition)
    ///     .badge_id(12345)
    ///     .points(5)
    ///     .build();
    /// game_data.add(achievement);
    ///
    /// let user_file = game_data.to_user_file();
    /// assert!(user_file.to_string().contains("First Step"));
    /// ```
    #[must_use]
    pub fn to_user_file(&self) -> UserFile {
        UserFile::new(
            self.title.clone(),
            self.iter_achievements().map(AchievementEntry::from),
            self.iter_leaderboards().map(LeaderboardEntry::from),
            self.iter_code_notes().map(CodeNoteEntry::from),
        )
    }
}

/// An asset for a game.
///
/// # Examples
///
/// ```
/// use rustcheevos::prelude::*;
/// use rustcheevos::types::{achievement::Achievement, game::GameAsset};
/// use rustcheevos::{chain, bits8};
///
/// // GameAsset can be created from Achievement, Leaderboard, or CodeNote
/// let condition = chain!(bits8!(0x1234).eq(1));
/// let achievement = Achievement::builder("First Step")
///     .description("Complete the tutorial")
///     .requirements(condition)
///     .badge_id(12345)
///     .points(5)
///     .build();
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

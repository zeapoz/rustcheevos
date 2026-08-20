//! Type definition for the core game container struct.

use crate::types::{
    achievement::Achievement, leaderboard::Leaderboard, note::CodeNote, rich::RichPresence,
};
use rustcheevos_schema::user::{CodeNoteEntry, UserFile};

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
///     .core(chain!(
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
///     .add_achievement(achievement)
///     .add_leaderboard(leaderboard)
///     .add_code_note(note)
///     .set_rich_presence(rich_presence);
///
/// // Serialize to the user file format.
/// let user_file = game_data.to_user_file("Rustcheevos");
/// println!("{user_file}");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct GameData {
    /// The game ID.
    id: u32,
    /// The game name.
    title: String,
    /// The achievements.
    achievements: Vec<Achievement>,
    /// The leaderboards.
    leaderboards: Vec<Leaderboard>,
    /// The code notes.
    code_notes: Vec<CodeNote>,
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
            achievements: Vec::new(),
            leaderboards: Vec::new(),
            code_notes: Vec::new(),
            rich_presence: RichPresence::new(),
        }
    }

    /// Adds an achievement to this game.
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
    ///     .core(chain!(bits8!(0x1234).eq(1)))
    ///     .badge_id(12345)
    ///     .points(5)
    ///     .build();
    ///
    /// game_data.add_achievement(achievement);
    /// assert_eq!(game_data.achievements().len(), 1);
    /// ```
    pub fn add_achievement(&mut self, achievement: Achievement) -> &mut Self {
        self.achievements.push(achievement);
        self
    }

    /// Adds achievements to this game.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustcheevos::prelude::*;
    /// # use rustcheevos::types::{achievement::Achievement, game::GameData};
    /// # use rustcheevos::{chain, bits8};
    /// let mut game_data = GameData::new(1, "Super Adventure");
    ///
    /// let a = Achievement::builder("Step A")
    ///     .description("Do A")
    ///     .core(chain!(bits8!(0x1234).eq(1)))
    ///     .badge_id(12345)
    ///     .points(5)
    ///     .build();
    /// let b = Achievement::builder("Step B")
    ///     .description("Do B")
    ///     .core(chain!(bits8!(0x1234).eq(1)))
    ///     .badge_id(12345)
    ///     .points(10)
    ///     .build();
    ///
    /// game_data.add_achievements([a, b]);
    /// assert_eq!(game_data.achievements().len(), 2);
    /// ```
    pub fn add_achievements(
        &mut self,
        achievements: impl IntoIterator<Item = Achievement>,
    ) -> &mut Self {
        self.achievements.extend(achievements);
        self
    }

    /// Adds a leaderboard to this game.
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
    /// game_data.add_leaderboard(leaderboard);
    /// assert_eq!(game_data.leaderboards().len(), 1);
    /// ```
    pub fn add_leaderboard(&mut self, leaderboard: Leaderboard) -> &mut Self {
        self.leaderboards.push(leaderboard);
        self
    }

    /// Adds leaderboards to this game.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustcheevos::prelude::*;
    /// # use rustcheevos::types::{game::GameData, leaderboard::{Leaderboard, LeaderboardFormat}};
    /// # use rustcheevos::{chain, bits8, measured};
    /// let mut game_data = GameData::new(1, "Super Adventure");
    ///
    /// let lb = Leaderboard::builder("Speed Run")
    ///     .description("Complete the game fast")
    ///     .start(chain!(bits8!(0x1234).eq(1)))
    ///     .cancel(chain!(bits8!(0x1234).eq(0)))
    ///     .submit(chain!(bits8!(0xABCD).eq(1)))
    ///     .value(measured!(bits8!(0xDEF0)))
    ///     .format(LeaderboardFormat::Seconds)
    ///     .lower_is_better(true)
    ///     .build();
    ///
    /// game_data.add_leaderboards([lb]);
    /// assert_eq!(game_data.leaderboards().len(), 1);
    /// ```
    pub fn add_leaderboards(
        &mut self,
        leaderboards: impl IntoIterator<Item = Leaderboard>,
    ) -> &mut Self {
        self.leaderboards.extend(leaderboards);
        self
    }

    /// Adds a code note to this game.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustcheevos::prelude::*;
    /// # use rustcheevos::types::{game::GameData, note::CodeNote};
    /// let mut game_data = GameData::new(1, "Super Adventure");
    ///
    /// let note = CodeNote::new(0x1234, "Player health");
    /// game_data.add_code_note(note);
    /// assert_eq!(game_data.code_notes().len(), 1);
    /// ```
    pub fn add_code_note(&mut self, note: CodeNote) -> &mut Self {
        self.code_notes.push(note);
        self
    }

    /// Adds code notes to this game.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustcheevos::prelude::*;
    /// # use rustcheevos::types::{game::GameData, note::CodeNote};
    /// let mut game_data = GameData::new(1, "Super Adventure");
    ///
    /// let a = CodeNote::new(0x1234, "Player health");
    /// let b = CodeNote::new(0x5678, "Player lives");
    /// game_data.add_code_notes([a, b]);
    /// assert_eq!(game_data.code_notes().len(), 2);
    /// ```
    pub fn add_code_notes(&mut self, notes: impl IntoIterator<Item = CodeNote>) -> &mut Self {
        self.code_notes.extend(notes);
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
    ///     .core(condition)
    ///     .badge_id(12345)
    ///     .points(5)
    ///     .build();
    /// game_data.add_achievement(achievement);
    ///
    /// let user_file = game_data.to_user_file("Rustcheevos");
    /// assert!(user_file.to_string().contains("First Step"));
    /// ```
    #[must_use]
    pub fn to_user_file(&self, author: impl Into<String>) -> UserFile {
        let author = author.into();
        UserFile::new(
            self.title.clone(),
            self.achievements.iter().map(|a| a.to_user_entry(&author)),
            self.leaderboards.iter().map(Leaderboard::to_user_entry),
            self.code_notes.iter().map(CodeNoteEntry::from),
        )
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

    /// Returns the rich presence for this game.
    #[must_use]
    pub fn rich_presence(&self) -> &RichPresence {
        &self.rich_presence
    }
}

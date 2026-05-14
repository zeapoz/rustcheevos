//! Leaderboard type definitions.

use std::fmt;
use std::str::FromStr;

use crate::parsers::ParseError;
use crate::prelude::Requirement;

use super::chain::ChainGroup;

/// A leaderboard definition.
///
/// This type defines the core properties of a leaderboard and is used to populate
/// an [`AchievementSet`][`crate::types::game::AchievementSet`].
///
/// # Examples
///
/// ```
/// # fn start_condition() -> Chain { Chain::default() }
/// # fn cancel_condition() -> Chain { Chain::default() }
/// # fn submit_condition() -> Chain { Chain::default() }
/// # use rustcheevos::bits8;
/// # fn value() -> MemoryRef { bits8!(0) }
/// use rustcheevos::{prelude::*, measured};
///
/// let leaderboard = Leaderboard::new(
///     "Speed Run",
///     "Complete the level as fast as possible",
///     start_condition(),
///     cancel_condition(),
///     submit_condition(),
///     measured!(value()),
///     LeaderboardFormat::Seconds,
///     true,
/// );
/// ```
///
/// [`Leaderboard::new()`] sets all of the required properties. To set ID, use
/// [`Leaderboard::with_id()`].
#[derive(Debug, Clone, PartialEq)]
pub struct Leaderboard {
    /// The leaderboard ID.
    pub id: u32,
    /// The leaderboard title.
    pub title: String,
    /// The leaderboard description.
    pub description: String,
    /// The leaderboard start condition.
    pub start: ChainGroup,
    /// The leaderboard cancel condition.
    pub cancel: ChainGroup,
    /// The leaderboard submit condition.
    pub submit: ChainGroup,
    /// The leaderboard value condition.
    pub value: ChainGroup,
    /// The value format.
    pub format: LeaderboardFormat,
    /// Whether lower values are better.
    pub lower_is_better: bool,
}

impl Leaderboard {
    /// Creates a new leaderboard with the given title, description, conditions, and format.
    ///
    /// # Examples
    /// ```
    /// # fn start_condition() -> Chain { Chain::default() }
    /// # fn cancel_condition() -> Chain { Chain::default() }
    /// # fn submit_condition() -> Chain { Chain::default() }
    /// # use rustcheevos::bits8;
    /// # fn value() -> MemoryRef { bits8!(0) }
    /// use rustcheevos::{prelude::*, measured};
    ///
    /// let leaderboard = Leaderboard::new(
    ///     "Speed Run",
    ///     "Complete the level as fast as possible",
    ///     start_condition(),
    ///     cancel_condition(),
    ///     submit_condition(),
    ///     measured!(value()),
    ///     LeaderboardFormat::Seconds,
    ///     true,
    /// );
    /// ```
    #[expect(clippy::too_many_arguments, reason = "all fields are required")]
    pub fn new(
        title: impl Into<String>,
        description: impl Into<String>,
        start: impl Into<ChainGroup>,
        cancel: impl Into<ChainGroup>,
        submit: impl Into<ChainGroup>,
        value: impl Into<ChainGroup>,
        format: LeaderboardFormat,
        lower_is_better: bool,
    ) -> Self {
        Self {
            id: 0,
            title: title.into(),
            description: description.into(),
            start: start.into(),
            cancel: cancel.into(),
            submit: submit.into(),
            value: value.into(),
            format,
            lower_is_better,
        }
    }

    // TODO: Adjust docs.
    /// Creates a new instant submission leaderboard.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::{bits8, measured};
    /// # fn start_condition() -> Chain { Chain::default() }
    /// # fn value() -> MemoryRef { bits8!(0) }
    /// use rustcheevos::prelude::*;
    ///
    /// let leaderboard = Leaderboard::new_instant_submission(
    ///     "Speed Run",
    ///     "Complete the level as fast as possible",
    ///     start_condition(),
    ///     measured!(value()),
    ///     LeaderboardFormat::Seconds,
    ///     true,
    /// );
    /// ```
    pub fn new_instant_submission(
        title: impl Into<String>,
        description: impl Into<String>,
        start: impl Into<ChainGroup>,
        value: impl Into<ChainGroup>,
        format: LeaderboardFormat,
        lower_is_better: bool,
    ) -> Self {
        Self::new(
            title,
            description,
            start,
            Requirement::always_false(),
            Requirement::always_true(),
            value,
            format,
            lower_is_better,
        )
    }

    /// Sets the leaderboard ID.
    ///
    /// # Examples
    /// ```
    /// # fn start_condition() -> Chain { Chain::default() }
    /// # fn cancel_condition() -> Chain { Chain::default() }
    /// # fn submit_condition() -> Chain { Chain::default() }
    /// # use rustcheevos::bits8;
    /// # fn value() -> MemoryRef { bits8!(0) }
    /// use rustcheevos::{prelude::*, measured};
    ///
    /// let leaderboard = Leaderboard::new(
    ///     "Speed Run",
    ///     "Complete the game as fast as possible",
    ///     start_condition(),
    ///     cancel_condition(),
    ///     submit_condition(),
    ///     measured!(value()),
    ///     LeaderboardFormat::Seconds,
    ///     true,
    /// )
    /// .with_id(600707);
    /// ```
    #[must_use]
    pub fn with_id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }
}

/// The format for a leaderboard value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LeaderboardFormat {
    /// Score format.
    Score,
    /// Seconds format.
    Seconds,
    /// Frames format.
    Frames,
    /// Milliseconds format.
    Milliseconds,
    /// Minutes format.
    Minutes,
    /// Seconds as minutes format.
    SecsAsMins,
    /// Raw value format.
    Value,
    /// Unsigned integer format.
    Unsigned,
    /// Tens format.
    Tens,
    /// Hundreds format.
    Hundreds,
    /// Thousands format.
    Thousands,
    /// Fixed 1 decimal place.
    Fixed1,
    /// Fixed 2 decimal places.
    Fixed2,
    /// Fixed 3 decimal places.
    Fixed3,
    /// Points format.
    Points,
    /// Custom format.
    Custom,
}

impl LeaderboardFormat {
    /// Returns the string representation of this format.
    ///
    /// # Returns
    ///
    /// The string representation.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            LeaderboardFormat::Score => "SCORE",
            LeaderboardFormat::Seconds => "SECONDS",
            LeaderboardFormat::Frames => "FRAMES",
            LeaderboardFormat::Milliseconds => "MILLISECONDS",
            LeaderboardFormat::Minutes => "MINUTES",
            LeaderboardFormat::SecsAsMins => "SECS_AS_MINS",
            LeaderboardFormat::Value => "VALUE",
            LeaderboardFormat::Unsigned => "UNSIGNED",
            LeaderboardFormat::Tens => "TENS",
            LeaderboardFormat::Hundreds => "HUNDREDS",
            LeaderboardFormat::Thousands => "THOUSANDS",
            LeaderboardFormat::Fixed1 => "FIXED1",
            LeaderboardFormat::Fixed2 => "FIXED2",
            LeaderboardFormat::Fixed3 => "FIXED3",
            LeaderboardFormat::Points => "POINTS",
            LeaderboardFormat::Custom => "CUSTOM",
        }
    }
}

impl fmt::Display for LeaderboardFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for LeaderboardFormat {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let format = match s.to_uppercase().as_str() {
            "SCORE" => Self::Score,
            "SECONDS" => Self::Seconds,
            "FRAMES" => Self::Frames,
            "MILLISECONDS" => Self::Milliseconds,
            "MINUTES" => Self::Minutes,
            "SECS_AS_MINS" => Self::SecsAsMins,
            "VALUE" => Self::Value,
            "UNSIGNED" => Self::Unsigned,
            "TENS" => Self::Tens,
            "HUNDREDS" => Self::Hundreds,
            "THOUSANDS" => Self::Thousands,
            "FIXED1" => Self::Fixed1,
            "FIXED2" => Self::Fixed2,
            "FIXED3" => Self::Fixed3,
            "POINTS" => Self::Points,
            "CUSTOM" => Self::Custom,
            s => return Err(ParseError::Leaderboard(s.to_string())),
        };
        Ok(format)
    }
}

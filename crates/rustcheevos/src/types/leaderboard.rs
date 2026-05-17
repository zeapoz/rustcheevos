//! Leaderboard type definitions.

use std::fmt;
use std::str::FromStr;

use crate::parsers::ParseError;
use crate::types::requirement::condition::Condition;

use super::chain::ChainGroup;

/// A leaderboard definition.
///
/// This type defines the core properties of a leaderboard and is used to populate
/// an [`AchievementSet`][`crate::types::game::AchievementSet`].
///
/// # Examples
///
/// ```
/// use rustcheevos::prelude::*;
/// use rustcheevos::types::leaderboard::{Leaderboard, LeaderboardFormat};
/// use rustcheevos::{bits8, measured};
///
/// let leaderboard = Leaderboard::builder("Speed Run")
///     .description("Complete the level as fast as possible")
///     .start(bits8!(0x1234).eq(1))
///     .cancel(bits8!(0x1234).eq(0))
///     .submit(bits8!(0xABCD).eq(1))
///     .value(measured!(bits8!(0xDEF0)))
///     .format(LeaderboardFormat::Seconds)
///     .lower_is_better(true)
///     .build();
/// ```
///
/// Use [`Leaderboard::builder()`] to construct a leaderboard.
#[derive(Debug, Clone, PartialEq)]
pub struct Leaderboard {
    /// The leaderboard ID.
    id: u32,
    /// The leaderboard title.
    title: String,
    /// The leaderboard description.
    description: String,
    /// The leaderboard start condition.
    start: ChainGroup,
    /// The leaderboard cancel condition.
    cancel: ChainGroup,
    /// The leaderboard submit condition.
    submit: ChainGroup,
    /// The leaderboard value condition.
    value: ChainGroup,
    /// The value format.
    format: LeaderboardFormat,
    /// Whether lower values are better.
    lower_is_better: bool,
}

impl Leaderboard {
    /// Returns the leaderboard ID.
    #[must_use]
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns the leaderboard title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the leaderboard description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the start condition.
    #[must_use]
    pub fn start(&self) -> &ChainGroup {
        &self.start
    }

    /// Returns the cancel condition.
    #[must_use]
    pub fn cancel(&self) -> &ChainGroup {
        &self.cancel
    }

    /// Returns the submit condition.
    #[must_use]
    pub fn submit(&self) -> &ChainGroup {
        &self.submit
    }

    /// Returns the value condition.
    #[must_use]
    pub fn value(&self) -> &ChainGroup {
        &self.value
    }

    /// Returns the format.
    #[must_use]
    pub fn format(&self) -> LeaderboardFormat {
        self.format
    }

    /// Returns whether lower values are better.
    #[must_use]
    pub fn lower_is_better(&self) -> bool {
        self.lower_is_better
    }

    /// Returns a builder for constructing a leaderboard.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::leaderboard::{Leaderboard, LeaderboardFormat};
    /// use rustcheevos::{bits8, measured};
    ///
    /// let leaderboard = Leaderboard::builder("Speed Run")
    ///     .description("Complete the level as fast as possible")
    ///     .start(bits8!(0x1234).eq(1))
    ///     .cancel(bits8!(0x1234).eq(0))
    ///     .submit(bits8!(0xABCD).eq(1))
    ///     .value(measured!(bits8!(0xDEF0)))
    ///     .format(LeaderboardFormat::Seconds)
    ///     .lower_is_better(true)
    ///     .id(600707)
    ///     .build();
    /// ```
    pub fn builder(title: impl Into<String>) -> LeaderboardBuilder {
        LeaderboardBuilder::new(title)
    }
}

/// A builder for constructing [`Leaderboard`] instances.
///
/// # Examples
///
/// ```
/// use rustcheevos::prelude::*;
/// use rustcheevos::types::leaderboard::{Leaderboard, LeaderboardFormat};
/// use rustcheevos::{bits8, measured};
///
/// let leaderboard = Leaderboard::builder("Speed Run")
///     .description("Complete the level as fast as possible")
///     .start(bits8!(0x1234).eq(1))
///     .cancel(bits8!(0x1234).eq(0))
///     .submit(bits8!(0xABCD).eq(1))
///     .value(measured!(bits8!(0xDEF0)))
///     .format(LeaderboardFormat::Seconds)
///     .lower_is_better(true)
///     .build();
/// ```
#[derive(Debug)]
pub struct LeaderboardBuilder {
    /// The title of the leaderboard.
    title: String,
    /// The description of the leaderboard.
    description: String,
    /// The leaderboard start condition.
    start: ChainGroup,
    /// The leaderboard cancel condition.
    cancel: ChainGroup,
    /// The leaderboard submit condition.
    submit: ChainGroup,
    /// The leaderboard value condition.
    value: ChainGroup,
    /// The leaderboard format.
    format: LeaderboardFormat,
    /// Whether lower values are better.
    lower_is_better: bool,
    /// The leaderboard ID.
    id: u32,
}

impl LeaderboardBuilder {
    /// Creates a new builder with the given title.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: String::new(),
            start: ChainGroup::from(Condition::always_true()),
            cancel: ChainGroup::from(Condition::always_false()),
            submit: ChainGroup::from(Condition::always_true()),
            value: ChainGroup::from(Condition::always_true()),
            format: LeaderboardFormat::Value,
            lower_is_better: false,
            id: 0,
        }
    }

    /// Sets the leaderboard description.
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Sets the leaderboard start condition.
    #[must_use]
    pub fn start(mut self, start: impl Into<ChainGroup>) -> Self {
        self.start = start.into();
        self
    }

    /// Sets the leaderboard cancel condition.
    #[must_use]
    pub fn cancel(mut self, cancel: impl Into<ChainGroup>) -> Self {
        self.cancel = cancel.into();
        self
    }

    /// Sets the leaderboard submit condition.
    #[must_use]
    pub fn submit(mut self, submit: impl Into<ChainGroup>) -> Self {
        self.submit = submit.into();
        self
    }

    /// Sets the leaderboard value condition.
    #[must_use]
    pub fn value(mut self, value: impl Into<ChainGroup>) -> Self {
        self.value = value.into();
        self
    }

    /// Sets the leaderboard value format.
    #[must_use]
    pub fn format(mut self, format: LeaderboardFormat) -> Self {
        self.format = format;
        self
    }

    /// Sets whether lower values are better.
    #[must_use]
    pub fn lower_is_better(mut self, lower_is_better: bool) -> Self {
        self.lower_is_better = lower_is_better;
        self
    }

    /// Sets the leaderboard ID.
    #[must_use]
    pub fn id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    /// Builds the leaderboard.
    #[must_use]
    pub fn build(self) -> Leaderboard {
        self.into()
    }
}

impl From<LeaderboardBuilder> for Leaderboard {
    fn from(builder: LeaderboardBuilder) -> Self {
        Self {
            id: builder.id,
            title: builder.title,
            description: builder.description,
            start: builder.start,
            cancel: builder.cancel,
            submit: builder.submit,
            value: builder.value,
            format: builder.format,
            lower_is_better: builder.lower_is_better,
        }
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

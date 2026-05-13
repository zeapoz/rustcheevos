use std::fmt;
use std::str::FromStr;

use crate::parsers::ParseError;

use super::chain::ChainGroup;

/// A leaderboard definition.
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
    /// Creates a new leaderboard.
    ///
    /// # Arguments
    ///
    /// * `title` - The leaderboard title.
    /// * `description` - The leaderboard description.
    /// * `start` - The leaderboard start condition.
    /// * `cancel` - The leaderboard cancel condition.
    /// * `submit` - The leaderboard submit condition.
    /// * `value` - The leaderboard value condition.
    /// * `format` - The value format.
    /// * `lower_is_better` - Whether lower values are better.
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

    /// Sets the leaderboard ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The leaderboard ID.
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
            s => return Err(ParseError::InvalidLeaderboardFormat(s.to_string())),
        };
        Ok(format)
    }
}

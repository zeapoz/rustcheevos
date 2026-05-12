use std::{fmt, str::FromStr};

use crate::ParseError;

use super::chain::ChainGroup;

/// An achievement definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Achievement {
    pub id: u32,
    /// The achievement title.
    pub title: String,
    /// The achievement description.
    pub description: String,
    /// The conditions that must be met for the achievement.
    pub requirements: ChainGroup,
    /// The achievement tag.
    pub tag: Tag,
    /// The point value.
    pub points: u32,
}

impl Achievement {
    /// Creates a new achievement with the given title, description, conditions, and points.
    ///
    /// # Arguments
    ///
    /// * `title` - The achievement title.
    /// * `description` - The achievement description.
    /// * `requirements` - The achievement conditions that must be met.
    /// * `points` - The point value.
    pub fn new(
        title: impl Into<String>,
        description: impl Into<String>,
        requirements: impl Into<ChainGroup>,
        points: u32,
    ) -> Self {
        Self {
            id: 0,
            title: title.into(),
            description: description.into(),
            requirements: requirements.into(),
            tag: Tag::default(),
            points,
        }
    }

    /// Sets the achievement ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The achievement ID.
    pub fn with_id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    /// Sets the achievement tag.
    ///
    /// # Arguments
    ///
    /// * `tag` - The achievement tag.
    pub fn with_tag(mut self, tag: Tag) -> Self {
        self.tag = tag;
        self
    }
}

/// Tags for achievements.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Tag {
    /// No special tag.
    #[default]
    None,
    /// Progression achievement.
    Progression,
    /// Win condition.
    WinCondition,
    /// Missable achievement.
    Missable,
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Tag::None => "",
            Tag::Progression => "progression",
            Tag::WinCondition => "win_condition",
            Tag::Missable => "missable",
        };
        write!(f, "{s}")
    }
}

impl FromStr for Tag {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tag = match s {
            "" => Self::None,
            "progression" => Self::Progression,
            "win_condition" => Self::WinCondition,
            "missable" => Self::Missable,
            s => return Err(ParseError::InvalidTag(s.to_string())),
        };
        Ok(tag)
    }
}

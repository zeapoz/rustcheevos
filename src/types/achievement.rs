//! Type definitions for achievements.

use std::{fmt, str::FromStr};

use crate::parsers::ParseError;

use super::chain::ChainGroup;

// TODO: Refactor into builder pattern.
/// An achievement definition.
///
/// This type defines the core properties of an achievement and is used to populate
/// an [`AchievementSet`][`crate::types::game::AchievementSet`].
///
/// # Examples
///
/// ```
/// # enum Galaxy { Alpha }
/// # enum Medal { Bronze }
/// # fn galaxy_all_medals_condition(galaxy: Galaxy, medal: Medal) -> Chain { Chain::default() }
/// use rustcheevos::prelude::*;
///
/// let achievement = Achievement::new(
///     "Alpha Amateur",
///     "Earn a Bronze medal or higher on every planet of the Alpha galaxy",
///     galaxy_all_medals_condition(Galaxy::Alpha, Medal::Bronze),
///     3,
/// );
/// ```
///
/// [`Achievement::new()`] sets all of the required properties and uses default values for optional
/// ones. To set ID or tag, you can use [`Achievement::with_id()`] and [`Achievement::with_tag()`].
#[derive(Debug, Clone, PartialEq)]
pub struct Achievement {
    /// The achievement ID.
    pub id: u32,
    /// The achievement title.
    pub title: String,
    /// The achievement description.
    pub description: String,
    /// The conditions that must be met for the achievement.
    pub requirements: ChainGroup,
    /// The achievement tag.
    pub tag: Option<Tag>,
    /// The point value.
    pub points: u32,
}

impl Achievement {
    /// Creates a new achievement with the given title, description, conditions, and points.
    ///
    /// # Examples
    /// ```
    /// # enum Galaxy { Alpha }
    /// # enum Medal { Bronze }
    /// # fn galaxy_all_medals_condition(galaxy: Galaxy, medal: Medal) -> Chain { Chain::default() }
    /// use rustcheevos::prelude::*;
    ///
    /// let achievement = Achievement::new(
    ///     "Alpha Amateur",
    ///     "Earn a Bronze medal or higher on every planet of the Alpha galaxy",
    ///     galaxy_all_medals_condition(Galaxy::Alpha, Medal::Bronze),
    ///     3,
    /// );
    /// ```
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
            tag: None,
            points,
        }
    }

    /// Sets the achievement ID.
    ///
    /// # Examples
    /// ```
    /// # enum Galaxy { Alpha }
    /// # enum Medal { Bronze }
    /// # fn galaxy_all_medals_condition(galaxy: Galaxy, medal: Medal) -> Chain { Chain::default() }
    /// use rustcheevos::prelude::*;
    ///
    /// let achievement = Achievement::new(
    ///     "Alpha Amateur",
    ///     "Earn a Bronze medal or higher on every planet of the Alpha galaxy",
    ///     galaxy_all_medals_condition(Galaxy::Alpha, Medal::Bronze),
    ///     3,
    /// )
    /// .with_id(600707);
    /// ```
    #[must_use]
    pub fn with_id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    /// Sets the achievement tag.
    ///
    /// # Examples
    /// ```
    /// # enum Galaxy { Alpha }
    /// # enum Medal { Bronze }
    /// # fn galaxy_all_medals_condition(galaxy: Galaxy, medal: Medal) -> Chain { Chain::default() }
    /// use rustcheevos::prelude::*;
    ///
    /// let achievement = Achievement::new(
    ///     "Alpha Amateur",
    ///     "Earn a Bronze medal or higher on every planet of the Alpha galaxy",
    ///     galaxy_all_medals_condition(Galaxy::Alpha, Medal::Bronze),
    ///     3,
    /// )
    /// .with_tag(Tag::Progression);
    /// ```
    #[must_use]
    pub fn with_tag(mut self, tag: Tag) -> Self {
        self.tag = Some(tag);
        self
    }
}

/// An achievement tag.
///
/// This enum defines all the unique tags that can be applied to an [Achievement].
/// Use with [`Achievement::with_tag`] to specify a tag.
///
/// # Examples
/// ```
/// # enum Medal { Bronze }
/// # fn all_medals_condition(medal: Medal) -> Chain { Chain::default() }
/// use rustcheevos::prelude::*;
///
/// let achievement = Achievement::new(
///     "Solar System Sentinel",
///     "Earn a Bronze medal or higher on every planet in every galaxy excluding the Lambda galaxy",
///     all_medals_condition(Medal::Bronze),
///     3,
/// )
/// .with_tag(Tag::WinCondition);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Tag {
    /// Represents a progression achievement.
    Progression,
    /// Represents ta win condition.
    WinCondition,
    /// Represents a missable achievement.
    Missable,
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
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
            "progression" => Self::Progression,
            "win_condition" => Self::WinCondition,
            "missable" => Self::Missable,
            s => return Err(ParseError::Tag(s.to_string())),
        };
        Ok(tag)
    }
}

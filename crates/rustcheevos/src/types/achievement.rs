//! Type definitions for achievements.

use std::{fmt, str::FromStr};

use crate::parsers::ParseError;

use super::chain::ChainGroup;
use super::requirement::condition::Condition;

/// An achievement definition.
///
/// This type defines the core properties of an achievement and is used to populate
/// an [`AchievementSet`][`crate::types::game::AchievementSet`].
///
/// # Examples
///
/// ```
/// use rustcheevos::prelude::*;
/// use rustcheevos::bits8;
///
/// let achievement = Achievement::builder("Alpha Amateur")
///     .description("Earn a Bronze medal or higher on every planet of the Alpha galaxy")
///     .requirements(bits8!(0x1234).eq(1))
///     .badge_id(12345)
///     .points(3)
///     .build();
/// ```
///
/// Use [`Achievement::builder()`] to construct an achievement.
#[derive(Debug, Clone, PartialEq)]
pub struct Achievement {
    /// The achievement ID.
    id: u32,
    /// The achievement title.
    title: String,
    /// The achievement description.
    description: String,
    /// The conditions that must be met for the achievement.
    requirements: ChainGroup,
    /// The achievement tag.
    tag: Option<Tag>,
    /// The point value.
    points: u32,
    /// The badge ID.
    badge_id: u32,
}

impl Achievement {
    /// Returns the achievement ID.
    #[must_use]
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns the achievement title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the achievement description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the achievement requirements.
    #[must_use]
    pub fn requirements(&self) -> &ChainGroup {
        &self.requirements
    }

    /// Returns the achievement tag.
    #[must_use]
    pub fn tag(&self) -> Option<&Tag> {
        self.tag.as_ref()
    }

    /// Returns the point value.
    #[must_use]
    pub fn points(&self) -> u32 {
        self.points
    }

    /// Returns the badge ID.
    #[must_use]
    pub fn badge_id(&self) -> u32 {
        self.badge_id
    }

    /// Returns a builder for constructing an achievement.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::bits8;
    ///
    /// let achievement = Achievement::builder("Alpha Amateur")
    ///     .description("Earn a Bronze medal or higher on every planet of the Alpha galaxy")
    ///     .requirements(bits8!(0x1234).eq(1))
    ///     .badge_id(12345)
    ///     .points(3)
    ///     .id(600707)
    ///     .tag(Tag::Progression)
    ///     .build();
    /// ```
    pub fn builder(title: impl Into<String>) -> AchievementBuilder {
        AchievementBuilder::new(title)
    }
}

/// A builder for constructing [`Achievement`] instances.
///
/// # Examples
///
/// ```
/// use rustcheevos::prelude::*;
/// use rustcheevos::bits8;
///
/// let achievement = Achievement::builder("Alpha Amateur")
///     .description("Earn a Bronze medal or higher on every planet of the Alpha galaxy")
///     .requirements(bits8!(0x1234).eq(1))
///     .badge_id(12345)
///     .points(3)
///     .id(600707)
///     .tag(Tag::Progression)
///     .build();
/// ```
#[derive(Debug)]
pub struct AchievementBuilder {
    /// The achievement title.
    title: String,
    /// The achievement description.
    description: String,
    /// The achievement requirements.
    requirements: ChainGroup,
    /// The achievement points.
    points: u32,
    /// The achievement ID.
    id: u32,
    /// The achievement badge ID.
    badge_id: u32,
    /// The achievement tag.
    tag: Option<Tag>,
}

impl AchievementBuilder {
    /// Creates a new builder with the given title.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: String::new(),
            requirements: ChainGroup::from(Condition::always_true()),
            points: 0,
            badge_id: 0,
            id: 0,
            tag: None,
        }
    }

    /// Sets the achievement description.
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Sets the achievement requirements.
    #[must_use]
    pub fn requirements(mut self, requirements: impl Into<ChainGroup>) -> Self {
        self.requirements = requirements.into();
        self
    }

    /// Sets the achievement point value.
    #[must_use]
    pub fn points(mut self, points: u32) -> Self {
        self.points = points;
        self
    }

    /// Sets the achievement ID.
    #[must_use]
    pub fn id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    /// Sets the achievement badge ID.
    #[must_use]
    pub fn badge_id(mut self, badge_id: u32) -> Self {
        self.badge_id = badge_id;
        self
    }

    /// Sets the achievement tag.
    #[must_use]
    pub fn tag(mut self, tag: Tag) -> Self {
        self.tag = Some(tag);
        self
    }

    /// Builds the achievement.
    #[must_use]
    pub fn build(self) -> Achievement {
        self.into()
    }
}

impl From<AchievementBuilder> for Achievement {
    fn from(builder: AchievementBuilder) -> Self {
        Self {
            id: builder.id,
            title: builder.title,
            description: builder.description,
            requirements: builder.requirements,
            tag: builder.tag,
            points: builder.points,
            badge_id: builder.badge_id,
        }
    }
}

/// An achievement tag.
///
/// This enum defines all the unique tags that can be applied to an [Achievement].
/// Use with [`AchievementBuilder::tag`] to specify a tag.
///
/// # Examples
/// ```
/// # enum Medal { Bronze }
/// # fn all_medals_condition(medal: Medal) -> Chain { Chain::default() }
/// use rustcheevos::prelude::*;
///
/// let achievement = Achievement::builder("Solar System Sentinel")
///     .description("Earn a Bronze medal or higher on every planet in every galaxy excluding the Lambda galaxy")
///     .requirements(all_medals_condition(Medal::Bronze))
///     .badge_id(12345)
///     .tag(Tag::WinCondition)
///     .build();
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

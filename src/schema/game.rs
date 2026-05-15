//! Definitions for game data files from the `RetroAchievements` API.

use serde::{Deserialize, Serialize};

/// The top-level game data response from the `RetroAchievements` API.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameData {
    /// Whether the request was successful.
    pub success: bool,
    /// The game ID.
    #[serde(rename = "GameId")]
    pub game_id: u32,
    /// The game title.
    pub title: String,
    /// URL to the game icon image.
    pub image_icon_url: String,
    /// The game ID used for rich presence.
    pub rich_presence_game_id: u32,
    /// The rich presence patch script.
    pub rich_presence_patch: String,
    /// The console ID.
    pub console_id: u32,
    /// The achievement sets for this game.
    pub sets: Vec<Set>,
}

/// An achievement set for a game.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Set {
    /// The set title, if any.
    pub title: Option<String>,
    /// The type of set (e.g. "core", "bonus").
    #[serde(rename = "Type")]
    pub kind: String,
    /// The achievement set ID.
    pub achievement_set_id: u32,
    /// The game ID this set belongs to.
    pub game_id: u32,
    /// URL to the set icon image.
    pub image_icon_url: String,
    /// The achievements in this set.
    pub achievements: Vec<Achievement>,
    /// The leaderboards in this set.
    pub leaderboards: Vec<Leaderboard>,
}

/// An individual achievement.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Achievement {
    /// The achievement ID.
    #[serde(rename = "ID")]
    pub id: u32,
    /// The memory address requirements string.
    pub mem_addr: String,
    /// The achievement title.
    pub title: String,
    /// The achievement description.
    pub description: String,
    /// The point value of the achievement.
    pub points: u32,
    /// The author of the achievement.
    pub author: String,
    /// Unix timestamp of when the achievement was last modified.
    pub modified: u64,
    /// Unix timestamp of when the achievement was created.
    pub created: u64,
    /// The badge name/identifier.
    pub badge_name: String,
    /// The achievement flags.
    pub flags: u32,
    /// The achievement category (e.g. `"progression"`, `"win_condition"`), if any.
    #[serde(rename = "Type")]
    pub category: Option<String>,
    /// The rarity percentage.
    pub rarity: f64,
    /// The hardcore rarity percentage.
    pub rarity_hardcore: f64,
    /// URL to the unlocked badge image.
    #[serde(rename = "BadgeURL")]
    pub badge_url: String,
    /// URL to the locked badge image.
    #[serde(rename = "BadgeLockedURL")]
    pub badge_locked_url: String,
}

/// An individual leaderboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Leaderboard {
    /// The leaderboard ID.
    #[serde(rename = "ID")]
    pub id: u32,
    /// The leaderboard memory definition string.
    pub mem: String,
    /// The display format (e.g. "VALUE", "TIME", "SCORE").
    pub format: String,
    /// Whether lower values are better.
    pub lower_is_better: bool,
    /// The leaderboard title.
    pub title: String,
    /// The leaderboard description.
    pub description: String,
    /// Whether the leaderboard is hidden.
    pub hidden: bool,
}

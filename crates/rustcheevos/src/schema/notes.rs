//! Definitions for code notes files from the `RetroAchievements` API.

use serde::{Deserialize, Serialize};

/// A single code note entry from the `RetroAchievements` API.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CodeNote {
    /// The user who created the note.
    pub user: String,
    /// The memory address in hex format.
    pub address: String,
    /// The note contents.
    pub note: String,
}

//! # Rustcheevos
//!
//! A library for building achievement sets and more for [`RetroAchievements`](https://retroachievements.org/)
//! programmatically using Rust.
//!
//! The core idea is to allow building achievements using small composable chains of conditions to reuse logic
//! while remaining in control of the output using a fluent chain-like API.
//!
//! ## Core Types
//!
//! - [`GameData`][`crate::types::game::GameData`] - Container for all game assets with export functionality
//! - [`Achievement`][`crate::types::achievement::Achievement`] - Achievement definitions with requirements and point values
//! - [`Leaderboard`][`crate::types::leaderboard::Leaderboard`] - Leaderboard definitions with start/cancel/submit/value conditions
//! - [`RichPresence`][`crate::types::rich::RichPresence`] - Dynamic display strings with lookup tables and conditions
//! - [`MemoryRef`][`crate::types::memory::MemoryRef`] - Memory location references (address, size, access mode)
//! - [`Requirement`][`crate::types::requirement::Requirement`] - Condition clauses (comparisons and arithmetic operations)
//! - [`Chain`][`crate::types::chain::Chain`] - Ordered requirement sequences with chaining operators
//! - [`PendingChain`][`crate::types::chain::PendingChain`] - Builder type for constructing chains fluently
//!
//! ## Example
//!
//! ```no_run
//! use rustcheevos::prelude::*;
//! use rustcheevos::types::{
//!     achievement::Achievement,
//!     chain::{Chain, PendingChain},
//!     game::GameData,
//!     memory::MemoryRef,
//!     rich::{Entry, LookupTable, RichPresence},
//! };
//! use rustcheevos::{add_address, bits8, chain, delta, measured};
//!
//! const GAME_ID: u32 = 20374;
//! const GAME_NAME: &str = "Geometry Wars: Galaxies";
//!
//! // Logic chains can be defined as functions.
//! fn in_game() -> Chain {
//!     chain!(bits8!(0x1234).eq(1))
//! }
//!
//! // Complex logic chains can be made simpler by returning pending chains that allow
//! // for modiyfing the last condition in the chain.
//! fn current_level() -> PendingChain<MemoryRef> {
//!     chain!(
//!         add_address!(bits8!(0x16).mul(2)),
//!         bits8!(0x2345),
//!     )
//! }
//!
//! // Use flags like delta! fluidly like you would in the achievement editor.
//! fn just_beat_level(level_id: u32) -> Chain {
//!     chain!(
//!         delta!(current_level()).eq(level_id),
//!         current_level().eq(level_id + 1),
//!     )
//! }
//!
//! fn main() {
//!     let mut game_data = GameData::new(GAME_ID, GAME_NAME);
//!
//!     // Define an achievement by combining conditions.
//!     let achievement = Achievement::builder("First Step")
//!         .description("Complete the tutorial level")
//!         .requirements(chain!(
//!             just_beat_level(1),
//!             in_game(),
//!         ))
//!         .badge_id(12345)
//!         .points(5)
//!         .build();
//!     game_data.add(achievement);
//!
//!     // Create a simple rich presence.
//!     let mut rich_presence = RichPresence::new();
//!
//!     // Register lookup tables.
//!     let table = LookupTable::new("Level")
//!         .with_entry(Entry::new(1, "Level 1"))
//!         .with_entry(Entry::new(2..=3, "Level 2"))
//!         .with_fallback("Main Menu");
//!
//!     // This returns a macro call handle that can be used directly in format! strings.
//!     let stage = rich_presence.register_lookup(table, bits8!(0x1234));
//!     rich_presence.add_static_display(format!("Currently in {stage}"));
//!
//!     game_data.set_rich_presence(rich_presence);
//!
//!     // Export to a directory.
//!     let directory = std::env::temp_dir().join("rustcheevos_example");
//!     game_data.export(&directory).unwrap();
//! }
//! ```

pub use rustcheevos_proc::chain;

mod macros;
pub(crate) mod parsers;
pub mod prelude;
pub mod types;
pub mod util;

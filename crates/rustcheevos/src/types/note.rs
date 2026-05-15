//! Code note type definitions.

/// A code note definition.
///
/// This type defines code notes for memory addresses and is used to populate
/// a [`GameData`][`crate::types::game::GameData`].
#[derive(Debug, Clone, PartialEq)]
pub struct CodeNote {
    /// The memory address.
    pub address: usize,
    /// The note contents.
    pub contents: String,
}

impl CodeNote {
    /// Creates a new code note with the given address and contents.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let note = CodeNote::new(0x1234, "Player health");
    /// ```
    pub fn new(address: usize, contents: impl Into<String>) -> Self {
        Self {
            address,
            contents: contents.into(),
        }
    }
}

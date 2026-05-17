//! Code note type definitions.

use std::num::ParseIntError;

use crate::schema::notes;
use crate::util::parse_hex_address;

/// A code note definition.
///
/// This type defines code notes for memory addresses and is used to populate
/// a [`GameData`][`crate::types::game::GameData`].
#[derive(Debug, Clone, PartialEq)]
pub struct CodeNote {
    /// The memory address.
    address: usize,
    /// The note contents.
    contents: String,
}

impl CodeNote {
    /// Returns the memory address.
    #[must_use]
    pub fn address(&self) -> usize {
        self.address
    }

    /// Returns the note contents.
    #[must_use]
    pub fn contents(&self) -> &str {
        &self.contents
    }

    /// Creates a new code note with the given address and contents.
    ///
    /// # Examples
    /// ```
    /// use rustcheevos::types::note::CodeNote;
    ///
    /// let note = CodeNote::new(0x1234, "Player health");
    /// assert_eq!(note.address(), 0x1234);
    /// assert_eq!(note.contents(), "Player health");
    /// ```
    pub fn new(address: usize, contents: impl Into<String>) -> Self {
        Self {
            address,
            contents: contents.into(),
        }
    }
}

impl TryFrom<notes::CodeNote> for CodeNote {
    type Error = ParseIntError;

    fn try_from(value: notes::CodeNote) -> Result<Self, Self::Error> {
        let address = parse_hex_address(&value.address)?;
        Ok(Self {
            address,
            contents: value.note,
        })
    }
}

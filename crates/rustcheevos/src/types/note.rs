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

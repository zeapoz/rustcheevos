//! Parsing logic for `RetroAchievements` code notes.

use color_eyre::eyre::{Result, eyre};
use rustcheevos::prelude::{CodeNote, MemorySize};

/// A code note that has been parsed into structured fields.
///
/// Contains the address, memory size, a human-readable title,
/// and the original note contents.
pub struct ParsedNote {
    /// The memory address from the original note.
    pub address: usize,
    /// The parsed memory size variant.
    pub size: MemorySize,
    /// The title extracted from the first line of the note.
    pub title: String,
    /// The full original note contents.
    pub contents: String,
}

impl ParsedNote {
    /// Parses a [`CodeNote`] into a structured [`ParsedNote`].
    pub fn try_from_code_note(note: &CodeNote) -> Result<Self> {
        let first_line = note
            .contents()
            .lines()
            .next()
            .ok_or_else(|| eyre!("empty note contents"))?;

        let (size, rest) = Self::parse_size(first_line)?;
        let title = rest.trim().to_string();
        if title.is_empty() {
            return Err(eyre!("no title found after size tag"));
        }

        Ok(Self {
            address: note.address(),
            size,
            title,
            contents: note.contents().to_string(),
        })
    }

    /// Extracts the size tag and remaining text from a line.
    fn parse_size(line: &str) -> Result<(MemorySize, &str)> {
        let mut rest = line;

        while let Some((inner, remaining)) = Self::extract_bracket_pair(rest) {
            if let Ok(size) = super::parse_memory_size(inner) {
                let (_, title) = Self::strip_leading_brackets(remaining);
                return Ok((size, title));
            }

            if remaining.trim_start().starts_with('[') {
                rest = remaining;
            } else {
                return Err(eyre!("no valid size tag found: [{inner}]"));
            }
        }

        Err(eyre!("no size tag found"))
    }

    /// Finds the first `[...]` bracket pair in a line.
    fn extract_bracket_pair(line: &str) -> Option<(&str, &str)> {
        let start = line.find('[')?;
        let end = line[start..].find(']')?;
        Some((&line[start + 1..start + end], &line[start + end + 1..]))
    }

    /// Strips consecutive leading bracket pairs from the remaining text.
    fn strip_leading_brackets(mut rest: &str) -> (&str, &str) {
        while let Some((_, remaining)) = Self::extract_bracket_pair(rest) {
            if remaining.trim_start().starts_with('[') {
                rest = remaining;
            } else {
                return (rest, remaining);
            }
        }
        ("", rest)
    }
}

/// Parses a slice of [`CodeNote`]s into [`ParsedNote`]s.
pub fn parse_notes(notes: &[CodeNote]) -> (Vec<ParsedNote>, usize) {
    let mut parsed_notes = Vec::new();
    let mut skipped = 0;

    for note in notes {
        match ParsedNote::try_from_code_note(note) {
            Ok(parsed) => parsed_notes.push(parsed),
            Err(e) => {
                skipped += 1;
                eprintln!("Skipping note at 0x{:x}: {e}", note.address());
            }
        }
    }

    (parsed_notes, skipped)
}

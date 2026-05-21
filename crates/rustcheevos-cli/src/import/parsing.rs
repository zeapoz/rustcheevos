//! Parsing logic for `RetroAchievements` code notes.

use rustcheevos::types::{memory::MemorySize, note::CodeNote};

use super::ValueStyle;

/// A code note that has been parsed into structured fields.
///
/// Contains the address, optional memory size, a human-readable title,
/// and the original note contents.
pub struct ParsedNote {
    /// The memory address from the original note.
    pub address: usize,
    /// The parsed memory size variant, if a recognized size tag was found.
    pub size: Option<MemorySize>,
    /// The title extracted from the first line of the note.
    pub title: String,
    /// The full original note contents.
    pub contents: String,
}

impl ParsedNote {
    /// Parses a [`CodeNote`] into a structured [`ParsedNote`].
    ///
    /// Returns `None` if the note has empty contents or no title after bracket tags.
    pub fn try_from_code_note(note: &CodeNote) -> Option<Self> {
        let first_line = note.contents().lines().next()?;

        let (size, rest) = Self::parse_size(first_line);
        let title = rest.trim().to_string();
        if title.is_empty() {
            return None;
        }

        Some(Self {
            address: note.address(),
            size,
            title,
            contents: note.contents().to_string(),
        })
    }

    /// Extracts the size tag and remaining text from a line.
    ///
    /// Returns `None` for size if no recognized size tag is found,
    /// but still extracts the title by stripping all leading bracket pairs.
    fn parse_size(line: &str) -> (Option<MemorySize>, &str) {
        let mut rest = line;

        while let Some((inner, remaining)) = Self::extract_bracket_pair(rest) {
            if let Ok(size) = super::parse_memory_size(inner) {
                let (_, title) = Self::strip_leading_brackets(remaining);
                return (Some(size), title);
            }

            if remaining.trim_start().starts_with('[') {
                rest = remaining;
            } else {
                let (_, title) = Self::strip_leading_brackets(remaining);
                return (None, title);
            }
        }

        (None, "")
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
///
/// When `value_style` is `Macro`, notes without a recognized size tag are skipped
/// with a warning. When `AddrOnly`, all notes are included regardless of size tag.
pub fn parse_notes(notes: &[CodeNote], value_style: &ValueStyle) -> (Vec<ParsedNote>, usize) {
    let mut parsed_notes = Vec::new();
    let mut skipped = 0;

    for note in notes {
        if let Some(parsed) = ParsedNote::try_from_code_note(note) {
            if *value_style == ValueStyle::Macro && parsed.size.is_none() {
                skipped += 1;
                eprintln!(
                    "Skipping note at 0x{:x}: no recognized size tag (use --value addr-only to include)",
                    note.address()
                );
            } else {
                parsed_notes.push(parsed);
            }
        } else {
            skipped += 1;
            eprintln!(
                "Skipping note at 0x{:x}: empty or malformed",
                note.address()
            );
        }
    }

    (parsed_notes, skipped)
}

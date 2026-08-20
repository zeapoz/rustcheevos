//! Code note preview.

use std::fmt;

use rustcheevos::types::note::CodeNote;

use crate::preview::format_separator;

/// A preview of a code note.
#[derive(Debug, Clone)]
pub struct CodeNotePreview<'a> {
    /// The code note to preview.
    pub note: &'a CodeNote,
}

impl fmt::Display for CodeNotePreview<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let note = self.note;
        writeln!(
            f,
            "{}",
            format_separator(&format!("Note: 0x{:04x}", note.address()))
        )?;
        writeln!(f, "{}", note.contents())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dump_example() {
        let note = CodeNote::new(0x1234, "[8-bit] Player health");
        println!(
            "\n--- code note preview ---\n{}",
            CodeNotePreview { note: &note }
        );
    }
}

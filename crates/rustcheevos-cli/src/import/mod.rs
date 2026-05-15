//! Import logic for converting `RetroAchievements` code notes into Rust code.

mod generator;
mod parsing;

use std::ops::{Range, RangeInclusive};
use std::path::Path;

use color_eyre::eyre::{Context, Result, eyre};
use rustcheevos::{prelude::CodeNote, schema, types::memory::MemorySize};

use generator::OutputGenerator;
use parsing::parse_notes;

/// Filter for selecting which code notes to process.
pub enum NoteFilter {
    /// Match a single note by address.
    Address(usize),
    /// Match notes within an inclusive address range.
    RangeInclusive(RangeInclusive<usize>),
    /// Match notes within an exclusive address range.
    Range(Range<usize>),
}

impl NoteFilter {
    /// Creates a filter matching a single note by hex address.
    pub fn address(s: &str) -> Result<Self> {
        Ok(NoteFilter::Address(parse_hex(s)?))
    }

    /// Creates a filter matching notes within a hex address range.
    ///
    /// Supports `start..=end` (inclusive) and `start..end` (exclusive).
    pub fn range(s: &str) -> Result<Self> {
        if let Some((start, end)) = s.split_once("..=") {
            Ok(NoteFilter::RangeInclusive(
                parse_hex(start)?..=parse_hex(end)?,
            ))
        } else if let Some((start, end)) = s.split_once("..") {
            Ok(NoteFilter::Range(parse_hex(start)?..parse_hex(end)?))
        } else {
            Err(eyre!("invalid range format: {s}"))
        }
    }

    /// Returns true if the given address matches this filter.
    pub fn matches(&self, address: usize) -> bool {
        match self {
            NoteFilter::Address(addr) => *addr == address,
            NoteFilter::RangeInclusive(range) => range.contains(&address),
            NoteFilter::Range(range) => range.contains(&address),
        }
    }
}

/// Parses a hex string (with optional 0x prefix) into a usize.
fn parse_hex(s: &str) -> Result<usize> {
    let s = s.trim();
    let s = s.strip_prefix("0x").unwrap_or(s);
    usize::from_str_radix(s, 16).map_err(|e| eyre!("invalid hex address '{s}': {e}"))
}

/// Imports code notes from the given JSON file and generates a Rust module.
pub fn import(
    input: &Path,
    output: &Path,
    add_doc_comments: bool,
    filter: Option<NoteFilter>,
) -> Result<()> {
    let notes = read_notes(input)?;
    let total_notes = notes.len();

    let notes = match filter {
        Some(filter) => notes
            .into_iter()
            .filter(|n| filter.matches(n.address))
            .collect(),
        None => notes,
    };

    let (parsed_notes, skipped) = parse_notes(&notes);

    let generated = OutputGenerator::new(add_doc_comments).generate(&parsed_notes)?;

    std::fs::write(output, generated)
        .with_context(|| format!("Failed to write {}", output.display()))?;

    if total_notes == notes.len() {
        println!(
            "Wrote {} function(s) to {} ({} skipped)",
            parsed_notes.len(),
            output.display(),
            skipped,
        );
    } else {
        println!(
            "Wrote {} function(s) to {} ({} skipped, filtered from {} notes)",
            parsed_notes.len(),
            output.display(),
            skipped,
            total_notes,
        );
    }

    Ok(())
}

/// Parses a size tag string into a [`MemorySize`] variant.
fn parse_memory_size(tag: &str) -> Result<MemorySize> {
    match tag.to_lowercase().as_str() {
        "bitflags" | "8-bit" => Ok(MemorySize::Bits8),
        "16-bit" => Ok(MemorySize::Bits16),
        "24-bit" => Ok(MemorySize::Bits24),
        "32-bit" => Ok(MemorySize::Bits32),
        "16-bit be" => Ok(MemorySize::Bits16BE),
        "24-bit be" => Ok(MemorySize::Bits24BE),
        "32-bit be" => Ok(MemorySize::Bits32BE),
        "bitcount" => Ok(MemorySize::BitCount),
        "float" => Ok(MemorySize::Float),
        "float be" => Ok(MemorySize::FloatBE),
        "double" => Ok(MemorySize::Double32),
        "double be" => Ok(MemorySize::Double32BE),
        "mbf32" => Ok(MemorySize::MBF32),
        "mbf32 le" => Ok(MemorySize::MBF32LE),
        "lower4" => Ok(MemorySize::Lower4),
        "upper4" => Ok(MemorySize::Upper4),
        other => Err(eyre!("unrecognized size tag: [{other}]")),
    }
}

/// Reads and deserializes code notes from a JSON file.
fn read_notes(path: &Path) -> Result<Vec<CodeNote>> {
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;

    let schema_notes: Vec<schema::notes::CodeNote> =
        serde_json::from_str(&contents).context("Failed to parse JSON")?;

    schema_notes
        .into_iter()
        .map(|n| {
            let address = n.address.clone();
            n.try_into()
                .with_context(|| format!("Invalid address: {address}"))
        })
        .collect()
}

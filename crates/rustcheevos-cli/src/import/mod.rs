//! Import logic for converting `RetroAchievements` code notes into Rust code.

mod generator;
mod parsing;

use std::fmt;
use std::ops::{Range, RangeInclusive};
use std::path::Path;

use clap::ValueEnum;
use color_eyre::eyre::{Context, Result, eyre};
use rustcheevos::{
    types::{memory::MemorySize, note::CodeNote},
    util::parse_hex_address,
};
use rustcheevos_schema::notes as notes_schema;

use generator::OutputGenerator;
use parsing::parse_notes;

/// Output format for generated Rust code.
#[derive(Debug, Clone, Default, ValueEnum)]
pub enum OutputFormat {
    /// Generate as `pub const fn` functions.
    #[default]
    Function,
    /// Generate as `pub const` constants.
    Const,
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::Function => write!(f, "function"),
            OutputFormat::Const => write!(f, "const"),
        }
    }
}

/// Value representation style for generated code.
#[derive(Debug, Clone, Default, PartialEq, ValueEnum)]
pub enum ValueStyle {
    /// Generate using memory accessor macros (e.g., `bits8!(0x1234)`).
    #[default]
    Macro,
    /// Generate raw address values only (e.g., `0x1234`).
    AddrOnly,
}

impl fmt::Display for ValueStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueStyle::Macro => write!(f, "macro"),
            ValueStyle::AddrOnly => write!(f, "addr-only"),
        }
    }
}

impl OutputFormat {
    /// Transforms a note title into the appropriate identifier name.
    pub fn transform_name(&self, title: &str) -> String {
        match self {
            OutputFormat::Function => heck::AsSnakeCase(title).to_string(),
            OutputFormat::Const => heck::AsShoutySnakeCase(title).to_string(),
        }
    }

    /// Formats a single code note item as Rust source code.
    pub fn format_item(&self, name: &str, value: &str, ret_type: &str) -> String {
        match self {
            OutputFormat::Function => {
                format!("pub const fn {name}() -> {ret_type} {{\n    {value}\n}}")
            }
            OutputFormat::Const => {
                format!("pub const {name}: {ret_type} = {value};")
            }
        }
    }
}

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
        Ok(NoteFilter::Address(
            parse_hex_address(s).with_context(|| format!("invalid hex address '{s}'"))?,
        ))
    }

    /// Creates a filter matching notes within a hex address range.
    ///
    /// Supports `start..=end` (inclusive) and `start..end` (exclusive).
    pub fn range(s: &str) -> Result<Self> {
        let (start_str, end_str, inclusive) = if let Some((start, end)) = s.split_once("..=") {
            (start, end, true)
        } else if let Some((start, end)) = s.split_once("..") {
            (start, end, false)
        } else {
            return Err(eyre!("invalid range format: {s}"));
        };

        let start = parse_hex_address(start_str)
            .with_context(|| format!("invalid hex address '{start_str}'"))?;
        let end = parse_hex_address(end_str)
            .with_context(|| format!("invalid hex address '{end_str}'"))?;

        if inclusive {
            Ok(NoteFilter::RangeInclusive(start..=end))
        } else {
            Ok(NoteFilter::Range(start..end))
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

/// Imports code notes from the given JSON file and generates a Rust module.
pub fn import(
    input: &Path,
    output: &Path,
    add_doc_comments: bool,
    filter: Option<NoteFilter>,
    format: OutputFormat,
    value_style: ValueStyle,
) -> Result<()> {
    let notes = read_notes(input)?;
    let total_notes = notes.len();

    let notes = match filter {
        Some(filter) => notes
            .into_iter()
            .filter(|n| filter.matches(n.address()))
            .collect(),
        None => notes,
    };

    let (parsed_notes, skipped) = parse_notes(&notes, &value_style);

    let item_type = match format {
        OutputFormat::Function => "function(s)",
        OutputFormat::Const => "constant(s)",
    };

    let generated =
        OutputGenerator::new(add_doc_comments, format, value_style).generate(&parsed_notes)?;

    std::fs::write(output, generated)
        .with_context(|| format!("Failed to write {}", output.display()))?;

    if total_notes == notes.len() {
        println!(
            "Wrote {} {} to {} ({} skipped)",
            parsed_notes.len(),
            item_type,
            output.display(),
            skipped,
        );
    } else {
        println!(
            "Wrote {} {} to {} ({} skipped, filtered from {} notes)",
            parsed_notes.len(),
            item_type,
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
        "8-bit" => Ok(MemorySize::Bits8),
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

    let schema_notes: Vec<notes_schema::CodeNote> =
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

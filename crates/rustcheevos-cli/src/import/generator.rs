//! Output generation for Rust memory reference code.

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Write;

use color_eyre::eyre::Result;
use rustcheevos::prelude::MemorySize;
use rustcheevos::types::memory::BitIndex;

use super::parsing::ParsedNote;

/// Generates Rust code from parsed code notes.
///
/// Produces a module containing `use` statements for required macros
/// and one function per parsed note that returns a [`MemoryRef`].
pub struct OutputGenerator {
    /// Whether to include doc comments in generated functions.
    add_doc_comments: bool,
}

impl OutputGenerator {
    /// Creates a new generator with the given configuration.
    pub fn new(add_doc_comments: bool) -> Self {
        Self { add_doc_comments }
    }

    /// Collects the set of macro names needed for the given notes.
    pub fn collect_used_macros(parsed_notes: &[ParsedNote]) -> HashSet<&'static str> {
        parsed_notes
            .iter()
            .map(|parsed| memory_size_to_macro(parsed.size))
            .collect()
    }

    /// Generates the `use` statements for the output module.
    pub fn generate_imports(used_macros: HashSet<&'static str>) -> Result<String> {
        let mut output = String::from("use rustcheevos::prelude::*;\n");

        let mut sorted_macros: Vec<_> = used_macros.into_iter().collect();
        sorted_macros.sort_unstable();
        if sorted_macros.len() == 1 {
            writeln!(output, "use rustcheevos::{};", sorted_macros[0])?;
        } else if !sorted_macros.is_empty() {
            writeln!(output, "use rustcheevos::{{{}}};", sorted_macros.join(", "))?;
        }
        output.push('\n');

        Ok(output)
    }

    /// Generates one function per parsed note.
    pub fn generate_functions(&self, parsed_notes: &[ParsedNote]) -> Result<String> {
        let mut output = String::new();
        let mut seen_names = HashMap::new();
        let mut name_counts: HashMap<String, usize> = HashMap::new();

        for parsed in parsed_notes {
            let base_name = heck::AsSnakeCase(&parsed.title).to_string();
            *name_counts.entry(base_name).or_insert(0) += 1;
        }

        for parsed in parsed_notes {
            let macro_name = memory_size_to_macro(parsed.size);
            let base_name = heck::AsSnakeCase(&parsed.title).to_string();
            let fn_name = Self::deduplicate_name(&mut seen_names, &name_counts, &base_name);
            let doc_comments = self.generate_doc_comments(&parsed.contents);

            writeln!(
                output,
                "{doc_comments}\
                pub const fn {fn_name}() -> MemoryRef {{\n    \
                    {macro_name}!(0x{:x})\n\
                }}\n",
                parsed.address
            )?;
        }

        Ok(output)
    }

    /// Formats note contents as Rust doc comments.
    fn generate_doc_comments(&self, contents: &str) -> String {
        if self.add_doc_comments {
            contents
                .lines()
                .map(|line| format!("/// {line}"))
                .collect::<Vec<_>>()
                .join("\n")
                + "\n"
        } else {
            String::new()
        }
    }

    /// Deduplicates function names by appending a numeric suffix when needed.
    fn deduplicate_name(
        seen_names: &mut HashMap<String, usize>,
        name_counts: &HashMap<String, usize>,
        base_name: &str,
    ) -> String {
        let count = name_counts.get(base_name).copied().unwrap_or(0);
        if count <= 1 {
            base_name.to_string()
        } else {
            let seen = seen_names.entry(base_name.to_string()).or_insert(0);
            *seen += 1;
            format!("{base_name}_{seen}")
        }
    }

    /// Generates the complete output module.
    pub fn generate(&self, parsed_notes: &[ParsedNote]) -> Result<String> {
        let used_macros = Self::collect_used_macros(parsed_notes);
        let imports = Self::generate_imports(used_macros)?;
        let functions = self.generate_functions(parsed_notes)?;

        Ok(format!("{imports}{functions}"))
    }
}

/// Maps a [`MemorySize`] variant to the corresponding macro name.
pub fn memory_size_to_macro(size: MemorySize) -> &'static str {
    match size {
        MemorySize::Bits8 => "bits8",
        MemorySize::Bits16 => "bits16",
        MemorySize::Bits24 => "bits24",
        MemorySize::Bits32 => "bits32",
        MemorySize::Bits16BE => "bits16be",
        MemorySize::Bits24BE => "bits24be",
        MemorySize::Bits32BE => "bits32be",
        MemorySize::BitCount => "bitcount",
        MemorySize::Float => "float",
        MemorySize::FloatBE => "floatbe",
        MemorySize::Double32 => "double32",
        MemorySize::Double32BE => "double32be",
        MemorySize::MBF32 => "mbf32",
        MemorySize::MBF32LE => "mbfle",
        MemorySize::BitIndex(index) => match index {
            BitIndex::Zero => "bit0",
            BitIndex::One => "bit1",
            BitIndex::Two => "bit2",
            BitIndex::Three => "bit3",
            BitIndex::Four => "bit4",
            BitIndex::Five => "bit5",
            BitIndex::Six => "bit6",
            BitIndex::Seven => "bit7",
        },
        MemorySize::Lower4 => "lower4",
        MemorySize::Upper4 => "upper4",
    }
}

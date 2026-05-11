use std::{fmt, fs, io, path::Path, rc::Rc};

use format::{Format, FormatType};
use lookup::LookupTable;
use macros::{MacroRef, builtin::BuiltInMacro};

use crate::schema::rich::{RICH_PESENCE_FILE_EXTENSION, RICH_PESENCE_FILE_SUFFIX};

use super::requirement::group::RequirementGroups;

pub mod format;
pub mod lookup;
pub mod macros;

/// The rich presence core type.
#[derive(Debug, Clone, PartialEq)]
pub struct RichPresence {
    lookup_tables: Vec<Rc<LookupTable>>,
    formats: Vec<Rc<Format>>,
    conditional_displays: Vec<ConditionalDisplay>,
    static_display: String,
}

impl RichPresence {
    /// Create a new rich presence.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new [`MacroRef`] for a builtin macro.
    pub fn builtin_macro(builtin: BuiltInMacro) -> MacroRef {
        MacroRef::builtin(builtin)
    }

    /// Registers a new lookup table and returns a [`MacroRef`] for it.
    ///
    /// # Arguments
    /// * `table` - The lookup table.
    pub fn register_lookup_table(&mut self, table: impl Into<LookupTable>) -> MacroRef {
        let idx = self.lookup_tables.len();
        self.lookup_tables.push(Rc::new(table.into()));
        MacroRef::Lookup(self.lookup_tables[idx].clone())
    }

    /// Registers a new format and returns a [`MacroRef`] for it.
    ///
    /// # Arguments
    /// * `name` - The name of the format.
    /// * `value` - The value of the format.
    /// * `format_type` - The format type.
    pub fn register_format(
        &mut self,
        name: impl Into<String>,
        format_type: FormatType,
    ) -> MacroRef {
        let format = Format::new(name.into(), format_type);
        let idx = self.formats.len();
        self.formats.push(Rc::new(format));
        MacroRef::format(self.formats[idx].clone())
    }

    /// Adds a conditional display.
    ///
    /// # Arguments
    /// * `condition` - The condition.
    /// * `display` - The conditional display.
    pub fn add_conditional_display(
        &mut self,
        conditions: impl Into<RequirementGroups>,
        display: impl Into<String>,
    ) {
        let display = ConditionalDisplay::new(conditions, display);
        self.conditional_displays.push(display);
    }

    /// Adds a static display.
    ///
    /// # Arguments
    /// * `display` - The formatted string.
    pub fn add_static_display(&mut self, display: impl Into<String>) {
        self.static_display = display.into();
    }

    /// Exports this set to to the rich presence file at the given directory and with the given game id.
    ///
    /// # Arguments
    ///
    /// * `game_id` - The game id.
    /// * `dir` - The directory to export to.
    pub fn export(&self, game_id: impl Into<String>, dir: impl AsRef<Path>) -> io::Result<()> {
        let filename = format!(
            "{}{}.{}",
            game_id.into(),
            RICH_PESENCE_FILE_SUFFIX,
            RICH_PESENCE_FILE_EXTENSION
        );
        let path = dir.as_ref().join(filename);
        self.export_to_file(path)
    }

    /// Exports this set to a custom file path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to export to.
    pub fn export_to_file(&self, path: impl AsRef<Path>) -> io::Result<()> {
        fs::write(path, self.to_string())
    }
}

impl Default for RichPresence {
    fn default() -> Self {
        Self {
            formats: Vec::new(),
            lookup_tables: Vec::new(),
            conditional_displays: Vec::new(),
            static_display: String::new(),
        }
    }
}

impl fmt::Display for RichPresence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for lookup_table in &self.lookup_tables {
            writeln!(f, "{}", lookup_table)?;
        }
        for format in &self.formats {
            writeln!(f, "{}", format)?;
        }
        writeln!(f, "Display:")?;
        for conditional_display in &self.conditional_displays {
            writeln!(f, "{}", conditional_display)?;
        }
        writeln!(f, "{}", self.static_display)
    }
}

/// A conditional rich presence display definition.
#[derive(Debug, Clone, PartialEq)]
pub struct ConditionalDisplay {
    pub condition: RequirementGroups,
    pub display: String,
}

impl ConditionalDisplay {
    /// Creates a new conditional display.
    pub fn new(condition: impl Into<RequirementGroups>, display: impl Into<String>) -> Self {
        Self {
            condition: condition.into(),
            display: display.into(),
        }
    }
}

impl fmt::Display for ConditionalDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "?{}?{}", self.condition, self.display)
    }
}

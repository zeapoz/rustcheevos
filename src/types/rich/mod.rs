//! Type definitions for rich presence.

use std::{fmt, fs, io, path::Path, rc::Rc};

use format::{Format, FormatType};
use lookup::LookupTable;
use macros::{MacroRef, builtin::BuiltInMacro};

use crate::schema::rich::{RICH_PESENCE_FILE_EXTENSION, RICH_PESENCE_FILE_SUFFIX};

use super::chain::ChainGroup;

pub mod format;
pub mod lookup;
pub mod macros;

/// The rich presence core type.
///
/// # Examples
///
/// ```
/// use rustcheevos::{prelude::*, chain, bits8};
/// use rustcheevos::types::rich::lookup::Entry;
///
/// let mut rich_presence = RichPresence::new();
///
/// // Register lookup tables.
/// let mut table = LookupTable::new("Stage");
/// table.add_entry(Entry::new(1, "Level 1"));
/// table.add_entry(Entry::new(2..=3, "Level 2"));
/// table.set_fallback("Main Menu");
///
/// let level = rich_presence.register_lookup_table(table).call(bits8!(0x1234));
///
/// // Add a conditional display based on game state.
/// let condition = chain!(bits8!(0x1234).ge(1));
/// rich_presence.add_conditional_display(condition, "Currently in {level}");
///
/// // Add a static display fallback.
/// rich_presence.add_static_display("Super Adventure - Main Menu");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RichPresence {
    /// The registered lookup tables.
    lookup_tables: Vec<Rc<LookupTable>>,
    /// The registered formats.
    formats: Vec<Rc<Format>>,
    /// The conditional displays.
    conditional_displays: Vec<ConditionalDisplay>,
    /// The static display.
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
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::{prelude::*, bits8};
    /// use rustcheevos::types::rich::lookup::Entry;
    ///
    /// let mut rich_presence = RichPresence::new();
    /// let mut table = LookupTable::new("Stage");
    /// table.add_entry(Entry::new(0, "Main Menu"));
    /// table.add_entry(Entry::new(1, "Level 1"));
    /// let stage = rich_presence.register_lookup_table(table).call(bits8!(0x1234));
    ///
    /// rich_presence.add_conditional_display(Requirement::always_true(), "Currently in {stage}");
    /// ```
    pub fn register_lookup_table(&mut self, table: impl Into<LookupTable>) -> MacroRef {
        let idx = self.lookup_tables.len();
        self.lookup_tables.push(Rc::new(table.into()));
        MacroRef::Lookup(self.lookup_tables[idx].clone())
    }

    /// Registers a new format and returns a [`MacroRef`] for it.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::{prelude::*, bits8};
    ///
    /// let mut rich_presence = RichPresence::new();
    /// let score = rich_presence.register_format("Score", FormatType::Score).call(bits8!(0x1234));
    ///
    /// rich_presence.add_conditional_display(Requirement::always_true(), "Score: {score}");
    /// ```
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
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::{prelude::*, chain, bits8};
    ///
    /// let mut rich_presence = RichPresence::new();
    /// let condition = chain!(bits8!(0x1234).ge(1));
    /// rich_presence.add_conditional_display(condition, "Playing a level");
    /// ```
    pub fn add_conditional_display(
        &mut self,
        conditions: impl Into<ChainGroup>,
        display: impl Into<String>,
    ) {
        let display = ConditionalDisplay::new(conditions, display);
        self.conditional_displays.push(display);
    }

    /// Adds a static display.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::prelude::*;
    ///
    /// let mut rich_presence = RichPresence::new();
    /// rich_presence.add_static_display("Super Adventure - Main Menu");
    /// ```
    pub fn add_static_display(&mut self, display: impl Into<String>) {
        self.static_display = display.into();
    }

    /// Exports this set to to the rich presence file at the given directory and with the given game id.
    ///
    /// # Errors
    ///
    /// Returns an error if the directory cannot be created or if writing fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustcheevos::prelude::*;
    ///
    /// let mut rich_presence = RichPresence::new();
    /// rich_presence.add_static_display("Super Adventure");
    ///
    /// let temp_dir = std::env::temp_dir().join("rustcheevos_rp_test");
    /// std::fs::create_dir_all(&temp_dir).unwrap();
    /// rich_presence.export("GAME001", &temp_dir).unwrap();
    /// ```
    pub fn export(&self, game_id: impl fmt::Display, dir: impl AsRef<Path>) -> io::Result<()> {
        let filename = format!("{game_id}{RICH_PESENCE_FILE_SUFFIX}.{RICH_PESENCE_FILE_EXTENSION}");
        let path = dir.as_ref().join(filename);
        self.export_to_file(path)
    }

    /// Exports this set to a custom file path.
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustcheevos::prelude::*;
    ///
    /// let mut rich_presence = RichPresence::new();
    /// rich_presence.add_static_display("Super Adventure");
    ///
    /// let temp_path = std::env::temp_dir().join("rich_presence.txt");
    /// rich_presence.export_to_file(&temp_path).unwrap();
    /// ```
    pub fn export_to_file(&self, path: impl AsRef<Path>) -> io::Result<()> {
        fs::write(path, self.to_string())
    }
}

impl fmt::Display for RichPresence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for lookup_table in &self.lookup_tables {
            writeln!(f, "{lookup_table}")?;
        }
        for format in &self.formats {
            writeln!(f, "{format}")?;
        }
        writeln!(f, "Display:")?;
        for conditional_display in &self.conditional_displays {
            writeln!(f, "{conditional_display}")?;
        }
        writeln!(f, "{}", self.static_display)
    }
}

/// A conditional rich presence display definition.
///
/// # Examples
///
/// ```
/// use rustcheevos::{prelude::*, chain, bits8};
/// use rustcheevos::types::rich::ConditionalDisplay;
///
/// let condition = chain!(bits8!(0x1234).ge(1));
/// let display = ConditionalDisplay::new(condition, "Playing a level");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ConditionalDisplay {
    /// The condition for displaying this display.
    pub condition: ChainGroup,
    /// The display to show when the condition is met.
    pub display: String,
}

impl ConditionalDisplay {
    /// Creates a new conditional display.
    pub fn new(condition: impl Into<ChainGroup>, display: impl Into<String>) -> Self {
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

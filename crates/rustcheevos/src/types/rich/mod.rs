//! Type definitions for rich presence.

use std::fmt;
use std::rc::Rc;

use format::Format;
use macros::MacroType;

use super::chain::ChainGroup;

mod format;
mod lookup;
mod macros;

pub use format::FormatType;
pub use lookup::{Entry, EntryKey, LookupTable};
pub use macros::builtin::BuiltInMacro;
pub use macros::{MacroDef, MacroRef, MacroValue};

/// The rich presence core type.
///
/// # Examples
///
/// ```
/// use rustcheevos::prelude::*;
/// use rustcheevos::types::rich::{LookupTable, Entry, RichPresence};
/// use rustcheevos::{chain, bits8};
///
/// let mut rich_presence = RichPresence::new();
///
/// // Register lookup tables.
/// let table = LookupTable::new("Stage")
///     .with_entry(Entry::new(1, "Level 1"))
///     .with_entry(Entry::new(2..=3, "Level 2"))
///     .with_fallback("Main Menu");
///
/// let level = rich_presence.register_lookup(table, bits8!(0x1234));
///
/// // Add a conditional display based on game state.
/// let condition = chain!(bits8!(0x1234).ge(1));
/// rich_presence.add_conditional_display(condition, format!("Currently in {level}"));
///
/// // Add a static display fallback.
/// rich_presence.add_static_display("Super Adventure - Main Menu");
///
/// // Serialize to the rich presence file format.
/// let output = rich_presence.to_string();
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
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a builtin macro and returns a [`MacroRef`] for it.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::rich::{BuiltInMacro, RichPresence};
    /// use rustcheevos::types::requirement::Condition;
    /// use rustcheevos::bits8;
    ///
    /// let mut rich_presence = RichPresence::new();
    /// let score = rich_presence.builtin_macro(BuiltInMacro::Score, bits8!(0x1234));
    ///
    /// rich_presence.add_conditional_display(Condition::always_true(), format!("Score: {score}"));
    /// ```
    pub fn builtin_macro(
        &mut self,
        builtin: BuiltInMacro,
        value: impl Into<MacroValue>,
    ) -> MacroRef {
        MacroRef::builtin(builtin, value)
    }

    /// Defines a format and returns a [`MacroDef`] for it.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::rich::{FormatType, RichPresence};
    /// use rustcheevos::types::requirement::Condition;
    /// use rustcheevos::bits8;
    ///
    /// let mut rich_presence = RichPresence::new();
    /// let score = rich_presence.define_format("Score", FormatType::Score).bind(bits8!(0x1234));
    ///
    /// rich_presence.add_conditional_display(Condition::always_true(), format!("Score: {score}"));
    /// ```
    pub fn define_format(&mut self, name: impl Into<String>, format_type: FormatType) -> MacroDef {
        let format = Format::new(name.into(), format_type);
        let idx = self.formats.len();
        self.formats.push(Rc::new(format));
        MacroDef::new(MacroType::Format(self.formats[idx].clone()))
    }

    /// Defines a lookup table and returns a [`MacroDef`] for it.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::rich::{LookupTable, Entry, RichPresence};
    /// use rustcheevos::types::requirement::Condition;
    /// use rustcheevos::bits8;
    ///
    /// let mut rich_presence = RichPresence::new();
    /// let table = LookupTable::new("Stage")
    ///     .with_entry(Entry::new(0, "Main Menu"))
    ///     .with_entry(Entry::new(1, "Level 1"));
    /// let stage = rich_presence.define_lookup(table).bind(bits8!(0x1234));
    ///
    /// rich_presence.add_conditional_display(Condition::always_true(), format!("Currently in {stage}"));
    /// ```
    pub fn define_lookup(&mut self, table: impl Into<LookupTable>) -> MacroDef {
        let idx = self.lookup_tables.len();
        self.lookup_tables.push(Rc::new(table.into()));
        MacroDef::new(MacroType::Lookup(self.lookup_tables[idx].clone()))
    }

    /// Registers a lookup table and returns a [`MacroRef`] for it.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::rich::{LookupTable, Entry, RichPresence};
    /// use rustcheevos::types::requirement::Condition;
    /// use rustcheevos::bits8;
    ///
    /// let mut rich_presence = RichPresence::new();
    /// let table = LookupTable::new("Stage")
    ///     .with_entry(Entry::new(0, "Main Menu"))
    ///     .with_entry(Entry::new(1, "Level 1"));
    /// let stage = rich_presence.register_lookup(table, bits8!(0x1234));
    ///
    /// rich_presence.add_conditional_display(Condition::always_true(), format!("Currently in {stage}"));
    /// ```
    pub fn register_lookup(
        &mut self,
        table: impl Into<LookupTable>,
        value: impl Into<MacroValue>,
    ) -> MacroRef {
        self.define_lookup(table).bind(value)
    }

    /// Registers a format and returns a [`MacroRef`] for it.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::rich::{FormatType, RichPresence};
    /// use rustcheevos::types::requirement::Condition;
    /// use rustcheevos::bits8;
    ///
    /// let mut rich_presence = RichPresence::new();
    /// let score = rich_presence.register_format("Score", FormatType::Score, bits8!(0x1234));
    ///
    /// rich_presence.add_conditional_display(Condition::always_true(), format!("Score: {score}"));
    /// ```
    pub fn register_format(
        &mut self,
        name: impl Into<String>,
        format_type: FormatType,
        value: impl Into<MacroValue>,
    ) -> MacroRef {
        self.define_format(name, format_type).bind(value)
    }

    /// Adds a conditional display.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::prelude::*;
    /// use rustcheevos::types::rich::RichPresence;
    /// use rustcheevos::{chain, bits8};
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
    /// use rustcheevos::types::rich::RichPresence;
    ///
    /// let mut rich_presence = RichPresence::new();
    /// rich_presence.add_static_display("Super Adventure - Main Menu");
    /// ```
    pub fn add_static_display(&mut self, display: impl Into<String>) {
        self.static_display = display.into();
    }

    /// Returns an iterator over the conditional displays.
    pub fn iter_conditional_displays(&self) -> impl Iterator<Item = &ConditionalDisplay> {
        self.conditional_displays.iter()
    }

    /// Returns the static display string.
    #[must_use]
    pub fn static_display(&self) -> &str {
        &self.static_display
    }

    /// Returns true if this rich presence has no content.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.lookup_tables.is_empty()
            && self.formats.is_empty()
            && self.conditional_displays.is_empty()
            && self.static_display.is_empty()
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
#[derive(Debug, Clone, PartialEq)]
pub struct ConditionalDisplay {
    /// The condition for displaying this display.
    condition: ChainGroup,
    /// The display to show when the condition is met.
    display: String,
}

impl ConditionalDisplay {
    /// Creates a new conditional display.
    pub fn new(condition: impl Into<ChainGroup>, display: impl Into<String>) -> Self {
        Self {
            condition: condition.into(),
            display: display.into(),
        }
    }

    /// Returns the display string.
    #[must_use]
    pub fn display(&self) -> &str {
        &self.display
    }
}

impl fmt::Display for ConditionalDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "?{}?{}", self.condition, self.display)
    }
}

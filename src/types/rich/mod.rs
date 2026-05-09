use std::rc::Rc;

use format::{Format, FormatType};
use lookup::{LookupTable, LookupTableHandle};
use macros::{MacroRef, MacroValue, builtin::BuiltInMacro};

use crate::prelude::RequirementGroup;

pub mod format;
pub mod lookup;
pub mod macros;

/// The rich presence core type.
#[derive(Debug, Clone, PartialEq)]
pub struct RichPresence {
    formats: Vec<Rc<Format>>,
    lookup_tables: Vec<LookupTableHandle>,
    conditional_displays: Vec<ConditionalDisplay>,
    static_display: String,
}

impl RichPresence {
    /// Create a new rich presence.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new [`MacroHandle`] for a builtin macro.
    pub fn builtin_macro(builtin: BuiltInMacro, value: impl Into<MacroValue>) -> MacroRef {
        MacroRef::builtin(builtin, value)
    }

    /// Registers a new format and returns a [`MacroHandle`] for it.
    ///
    /// # Arguments
    /// * `name` - The name of the format.
    /// * `value` - The value of the format.
    /// * `format_type` - The format type.
    pub fn register_format(
        &mut self,
        name: impl Into<String>,
        value: impl Into<MacroValue>,
        format_type: FormatType,
    ) -> MacroRef {
        let format = Format::new(name.into(), format_type);
        let idx = self.formats.len();
        self.formats.push(Rc::new(format));
        MacroRef::format(self.formats[idx].clone(), value)
    }

    /// Registers a new lookup table and returns a [`MacroHandle`] for it.
    ///
    /// # Arguments
    /// * `table` - The lookup table.
    pub fn register_lookup_table(&mut self, table: impl Into<LookupTable>) -> LookupTableHandle {
        let idx = self.lookup_tables.len();
        self.lookup_tables
            .push(LookupTableHandle::new(table.into()));
        self.lookup_tables[idx].clone()
    }

    /// Adds a conditional display.
    ///
    /// # Arguments
    /// * `condition` - The condition.
    /// * `display` - The conditional display.
    pub fn add_conditional_display(
        &mut self,
        condition: impl Into<RequirementGroup>,
        display: impl Into<String>,
    ) {
        let display = ConditionalDisplay::new(condition, display);
        self.conditional_displays.push(display);
    }

    /// Adds a static display.
    ///
    /// # Arguments
    /// * `display` - The formatted string.
    pub fn add_static_display(&mut self, display: impl Into<String>) {
        self.static_display = display.into();
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

/// A conditional rich presence display definition.
#[derive(Debug, Clone, PartialEq)]
pub struct ConditionalDisplay {
    pub condition: RequirementGroup,
    pub display: String,
}

impl ConditionalDisplay {
    /// Creates a new conditional display.
    pub fn new(condition: impl Into<RequirementGroup>, display: impl Into<String>) -> Self {
        Self {
            condition: condition.into(),
            display: display.into(),
        }
    }
}

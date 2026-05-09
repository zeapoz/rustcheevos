use std::{collections::HashMap, fmt, ops::RangeInclusive, rc::Rc};

use super::macros::{MacroRef, MacroValue};

/// A lookup table for rich presence data.
#[derive(Debug, Clone, PartialEq)]
pub struct LookupTable {
    pub name: String,
    pub entries: Vec<Entry>,
    pub fallback: Option<String>,
}

impl LookupTable {
    /// Creates a new lookup table with the given name.
    pub fn new(name: String) -> Self {
        Self {
            name,
            entries: Vec::new(),
            fallback: None,
        }
    }

    /// Adds an entry to the lookup table.
    ///
    /// # Arguments
    /// * `entry` - The entry to add.
    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    /// Adds multiple entries to the lookup table.
    ///
    /// # Arguments
    /// * `entries` - The entries to add.
    pub fn add_entries(&mut self, entries: Vec<Entry>) {
        self.entries.extend(entries);
    }

    /// Sets the fallback value for the lookup table.
    ///     
    /// # Arguments
    /// * `fallback` - The fallback value.
    pub fn set_fallback(&mut self, fallback: String) {
        self.fallback = Some(fallback);
    }
}

impl<T, S> From<Vec<(T, S)>> for LookupTable
where
    T: Into<EntryKey>,
    S: Into<String>,
{
    fn from(value: Vec<(T, S)>) -> Self {
        let mut entries = Vec::new();
        for (key, value) in value {
            entries.push(Entry::new(key, value.into()));
        }
        Self {
            name: String::new(),
            entries,
            fallback: None,
        }
    }
}

impl<T, S> From<HashMap<T, S>> for LookupTable
where
    T: Into<EntryKey>,
    S: Into<String>,
{
    fn from(value: HashMap<T, S>) -> Self {
        let mut entries = Vec::new();
        for (key, value) in value {
            entries.push(Entry::new(key, value.into()));
        }
        Self {
            name: String::new(),
            entries,
            fallback: None,
        }
    }
}

impl fmt::Display for LookupTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Lookup:{}", self.name)?;
        for entry in &self.entries {
            writeln!(f, "{}", entry)?;
        }
        if let Some(fallback) = &self.fallback {
            writeln!(f, "*={}", fallback)?;
        }
        Ok(())
    }
}

/// A lookup table handle.
#[derive(Debug, Clone, PartialEq)]
pub struct LookupTableHandle(Rc<LookupTable>);

impl LookupTableHandle {
    /// Returns a new [`LookupTableHandle`] for this lookup table.
    pub fn new(table: impl Into<LookupTable>) -> Self {
        Self(Rc::new(table.into()))
    }

    /// Returns a macro reference for the given key.
    ///
    /// # Arguments
    /// * `key` - The key.
    pub fn lookup(&self, key: impl Into<MacroValue>) -> MacroRef {
        MacroRef::lookup(self.0.clone(), key.into())
    }
}

/// An entry in a lookup table.
#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    pub keys: Vec<EntryKey>,
    pub value: String,
}

impl Entry {
    /// Creates a new entry with a given key and value.
    ///
    /// # Arguments
    /// * `key` - The key.
    /// * `value` - The value.
    pub fn new(key: impl Into<EntryKey>, value: String) -> Self {
        Self {
            keys: vec![key.into()],
            value,
        }
    }

    /// Adds an alternate key to the entry.
    ///     
    /// # Arguments
    /// * `key` - The key.
    pub fn add_key(&mut self, key: impl Into<EntryKey>) {
        self.keys.push(key.into());
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let keys = self
            .keys
            .iter()
            .map(|k| k.to_string())
            .collect::<Vec<_>>()
            .join(",");
        write!(f, "{}={}", keys, self.value)
    }
}

/// A key in a lookup table.
#[derive(Debug, Clone, PartialEq)]
pub enum EntryKey {
    /// A single value.
    Value(u32),
    /// A range of values.
    Range(RangeInclusive<u32>),
}

impl From<u32> for EntryKey {
    fn from(value: u32) -> Self {
        Self::Value(value)
    }
}

impl From<RangeInclusive<u32>> for EntryKey {
    fn from(range: RangeInclusive<u32>) -> Self {
        Self::Range(range)
    }
}

impl fmt::Display for EntryKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Value(value) => write!(f, "{}", value),
            Self::Range(range) => write!(f, "{}-{}", range.start(), range.end()),
        }
    }
}

use std::{fmt, ops::RangeInclusive};

/// A lookup table for rich presence data.
#[derive(Debug, Clone, PartialEq)]
pub struct LookupTable {
    pub name: String,
    pub entries: Vec<Entry>,
    pub fallback: Option<String>,
}

impl LookupTable {
    /// Creates a new lookup table with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            entries: Vec::new(),
            fallback: None,
        }
    }

    /// Creates a new lookup table with the given name and entries.
    pub fn from_iter(
        name: impl Into<String>,
        entries: impl IntoIterator<Item = impl Into<Entry>>,
    ) -> Self {
        Self {
            name: name.into(),
            entries: entries.into_iter().map(|e| e.into()).collect(),
            fallback: None,
        }
    }

    /// Adds an entry to the lookup table.
    ///
    /// # Arguments
    /// * `entry` - The entry to add.
    pub fn add_entry(&mut self, entry: impl Into<Entry>) {
        self.entries.push(entry.into());
    }

    /// Adds multiple entries to the lookup table.
    ///
    /// # Arguments
    /// * `entries` - The entries to add.
    pub fn add_entries(&mut self, entries: impl IntoIterator<Item = Entry>) {
        self.entries.extend(entries);
    }

    /// Sets the fallback value for the lookup table.
    ///     
    /// # Arguments
    /// * `fallback` - The fallback value.
    pub fn set_fallback(&mut self, fallback: impl Into<String>) {
        self.fallback = Some(fallback.into());
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
    pub fn new(key: impl Into<EntryKey>, value: impl Into<String>) -> Self {
        Self {
            keys: vec![key.into()],
            value: value.into(),
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

impl<K, V> From<(K, V)> for Entry
where
    K: Into<EntryKey>,
    V: Into<String>,
{
    fn from(value: (K, V)) -> Self {
        Self::new(value.0, value.1)
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

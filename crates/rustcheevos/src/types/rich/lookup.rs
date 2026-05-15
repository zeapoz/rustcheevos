//! Lookup table types for rich presence display.

use std::{fmt, ops::RangeInclusive};

/// A lookup table for rich presence display values.
///
/// # Examples
///
/// ```
/// use rustcheevos::types::rich::lookup::{LookupTable, Entry, EntryKey};
///
/// let table = LookupTable::from_iter("Health", [
///     Entry::new(EntryKey::Value(0), "Dead"),
///     Entry::new(EntryKey::Range(1..=50), "Low"),
///     Entry::new(EntryKey::Range(51..=100), "Full"),
/// ]);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct LookupTable {
    /// The name of the lookup table.
    pub name: String,
    /// The entries in the lookup table.
    pub entries: Vec<Entry>,
    /// The fallback value when no entry matches.
    pub fallback: Option<String>,
}

impl LookupTable {
    /// Creates a new empty lookup table with the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::types::rich::lookup::LookupTable;
    ///
    /// let table = LookupTable::new("Health");
    /// assert_eq!(table.name, "Health");
    /// assert!(table.entries.is_empty());
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            entries: Vec::new(),
            fallback: None,
        }
    }

    /// Creates a new lookup table with the given name and entries.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::types::rich::lookup::{LookupTable, Entry};
    ///
    /// let table = LookupTable::from_iter("Health", [
    ///     (1, "Full"),
    ///     (2, "Empty"),
    /// ]);
    /// assert_eq!(table.entries.len(), 2);
    /// ```
    pub fn from_iter(
        name: impl Into<String>,
        entries: impl IntoIterator<Item = impl Into<Entry>>,
    ) -> Self {
        Self {
            name: name.into(),
            entries: entries.into_iter().map(Into::into).collect(),
            fallback: None,
        }
    }

    /// Adds an entry to the lookup table.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::types::rich::lookup::{LookupTable, Entry};
    ///
    /// let mut table = LookupTable::new("Health");
    /// table.add_entry(Entry::new(0, "Dead"));
    /// assert_eq!(table.entries.len(), 1);
    /// ```
    pub fn add_entry(&mut self, entry: impl Into<Entry>) {
        self.entries.push(entry.into());
    }

    /// Adds multiple entries to the lookup table.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::types::rich::lookup::{LookupTable, Entry};
    ///
    /// let mut table = LookupTable::new("Health");
    /// table.add_entries([
    ///     Entry::new(0, "Dead"),
    ///     Entry::new(1, "Alive"),
    /// ]);
    /// assert_eq!(table.entries.len(), 2);
    /// ```
    pub fn add_entries(&mut self, entries: impl IntoIterator<Item = Entry>) {
        self.entries.extend(entries);
    }

    /// Sets the fallback value for the lookup table.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::types::rich::lookup::LookupTable;
    ///
    /// let mut table = LookupTable::new("Health");
    /// table.set_fallback("Unknown");
    /// assert_eq!(table.fallback, Some("Unknown".to_string()));
    /// ```
    pub fn set_fallback(&mut self, fallback: impl Into<String>) {
        self.fallback = Some(fallback.into());
    }
}

impl fmt::Display for LookupTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Lookup:{}", self.name)?;
        for entry in &self.entries {
            writeln!(f, "{entry}")?;
        }
        if let Some(fallback) = &self.fallback {
            writeln!(f, "*={fallback}")?;
        }
        Ok(())
    }
}

/// An entry in a lookup table mapping keys to display values.
#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    /// The keys that map to this entry.
    pub keys: Vec<EntryKey>,
    /// The display value for this entry.
    pub value: String,
}

impl Entry {
    /// Creates a new entry with a given key and value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::types::rich::lookup::Entry;
    ///
    /// let entry = Entry::new(1, "Full Health");
    /// assert_eq!(entry.value, "Full Health");
    /// ```
    pub fn new(key: impl Into<EntryKey>, value: impl Into<String>) -> Self {
        Self {
            keys: vec![key.into()],
            value: value.into(),
        }
    }

    /// Adds an alternate key to the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::types::rich::lookup::Entry;
    ///
    /// let mut entry = Entry::new(1, "Full Health");
    /// entry.add_key(100);  // Alternate key for max health
    /// assert_eq!(entry.keys.len(), 2);
    /// ```
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

impl<K, V> From<&(K, V)> for Entry
where
    K: Into<EntryKey> + Clone,
    V: Into<String> + Clone,
{
    fn from(value: &(K, V)) -> Self {
        Self::new(value.0.clone(), value.1.clone())
    }
}
impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let keys = self
            .keys
            .iter()
            .map(ToString::to_string)
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
            Self::Value(value) => write!(f, "{value}"),
            Self::Range(range) => write!(f, "{}-{}", range.start(), range.end()),
        }
    }
}

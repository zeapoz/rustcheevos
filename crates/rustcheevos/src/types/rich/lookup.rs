//! Lookup table types for rich presence display.

use std::{fmt, ops::RangeInclusive};

/// A lookup table for rich presence display values.
///
/// # Examples
///
/// ```
/// use rustcheevos::types::rich::lookup::{LookupTable, Entry, EntryKey};
///
/// let table = LookupTable::new("Health")
///     .with_entry(Entry::new(EntryKey::Value(0), "Dead"))
///     .with_entry(Entry::new(EntryKey::Range(1..=50), "Low"))
///     .with_entry(Entry::new(EntryKey::Range(51..=100), "Full"));
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
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            entries: Vec::new(),
            fallback: None,
        }
    }

    /// Adds entries to the lookup table, returning `self` for chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::types::rich::lookup::LookupTable;
    ///
    /// let table = LookupTable::new("Health")
    ///     .with_entries([(1, "Full"), (2, "Empty")]);
    /// assert_eq!(table.entries.len(), 2);
    /// ```
    #[must_use]
    pub fn with_entries(mut self, entries: impl IntoIterator<Item = impl Into<Entry>>) -> Self {
        self.entries.extend(entries.into_iter().map(Into::into));
        self
    }

    /// Adds an entry to the lookup table, returning `self` for chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::types::rich::lookup::{LookupTable, Entry};
    ///
    /// let table = LookupTable::new("Health")
    ///     .with_entry(Entry::new(0, "Dead"));
    /// assert_eq!(table.entries.len(), 1);
    /// ```
    #[must_use]
    pub fn with_entry(mut self, entry: impl Into<Entry>) -> Self {
        self.entries.push(entry.into());
        self
    }

    /// Sets the fallback value for the lookup table, returning `self` for chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::types::rich::lookup::LookupTable;
    ///
    /// let table = LookupTable::new("Health")
    ///     .with_fallback("Unknown");
    /// assert_eq!(table.fallback, Some("Unknown".to_string()));
    /// ```
    #[must_use]
    pub fn with_fallback(mut self, fallback: impl Into<String>) -> Self {
        self.fallback = Some(fallback.into());
        self
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
    /// use rustcheevos::types::rich::lookup::{Entry, EntryKey};
    ///
    /// let entry = Entry::new(1, "Full Health");
    /// assert_eq!(entry.value, "Full Health");
    ///
    /// let range_entry = Entry::new(EntryKey::Range(1..=100), "Health Range");
    /// assert_eq!(range_entry.value, "Health Range");
    /// ```
    pub fn new(key: impl Into<EntryKey>, value: impl Into<String>) -> Self {
        Self {
            keys: vec![key.into()],
            value: value.into(),
        }
    }

    /// Adds an alternate key to the entry, returning `self` for chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::types::rich::lookup::Entry;
    ///
    /// let entry = Entry::new(1, "Full Health").with_key(100);
    /// assert_eq!(entry.keys.len(), 2);
    /// ```
    #[must_use]
    pub fn with_key(mut self, key: impl Into<EntryKey>) -> Self {
        self.keys.push(key.into());
        self
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

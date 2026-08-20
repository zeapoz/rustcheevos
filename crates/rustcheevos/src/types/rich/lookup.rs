//! Lookup table types for rich presence display.

use std::{fmt, ops::RangeInclusive};

/// A lookup table for rich presence display values.
///
/// # Examples
///
/// ```
/// use rustcheevos::types::rich::{LookupTable, Entry, EntryKey};
///
/// let table = LookupTable::new("Health")
///     .with_entry(Entry::new(EntryKey::Value(0), "Dead"))
///     .with_entry(Entry::new(EntryKey::Range(1..=50), "Low"))
///     .with_entry(Entry::new(EntryKey::Range(51..=100), "Full"));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct LookupTable {
    /// The name of the lookup table.
    name: String,
    /// The entries in the lookup table.
    entries: Vec<Entry>,
    /// The fallback value when no entry matches.
    fallback: Option<String>,
}

impl LookupTable {
    /// Creates a new empty lookup table with the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::types::rich::LookupTable;
    ///
    /// let table = LookupTable::new("Health");
    /// assert_eq!(table.name(), "Health");
    /// assert_eq!(table.entries().count(), 0);
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
    /// Entries with the same display value are automatically merged into a single
    /// entry with their keys combined, and contiguous key ranges are collapsed.
    /// Entries are sorted by minimum key value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::types::rich::LookupTable;
    ///
    /// let table = LookupTable::new("Health")
    ///     .with_entries([(1, "Full"), (2, "Empty")]);
    /// assert_eq!(table.entries().count(), 2);
    /// ```
    #[must_use]
    pub fn with_entries(mut self, entries: impl IntoIterator<Item = impl Into<Entry>>) -> Self {
        self.entries.extend(entries.into_iter().map(Into::into));
        self.entries = optimize_entries(&self.entries);
        self
    }

    /// Adds an entry to the lookup table, returning `self` for chaining.
    ///
    /// Entries with the same display value are automatically merged into a single
    /// entry with their keys combined, and contiguous key ranges are collapsed.
    /// Entries are sorted by minimum key value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::types::rich::{LookupTable, Entry};
    ///
    /// let table = LookupTable::new("Health")
    ///     .with_entry(Entry::new(0, "Dead"));
    /// assert_eq!(table.entries().count(), 1);
    /// ```
    #[must_use]
    pub fn with_entry(mut self, entry: impl Into<Entry>) -> Self {
        self.entries.push(entry.into());
        self.entries = optimize_entries(&self.entries);
        self
    }

    /// Sets the fallback value for the lookup table, returning `self` for chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::types::rich::LookupTable;
    ///
    /// let table = LookupTable::new("Health")
    ///     .with_fallback("Unknown");
    /// assert_eq!(table.fallback(), Some("Unknown"));
    /// ```
    #[must_use]
    pub fn with_fallback(mut self, fallback: impl Into<String>) -> Self {
        self.fallback = Some(fallback.into());
        self
    }

    /// Returns the name of the lookup table.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the entries in the lookup table.
    pub fn entries(&self) -> impl Iterator<Item = &Entry> {
        self.entries.iter()
    }

    /// Returns the fallback value.
    #[must_use]
    pub fn fallback(&self) -> Option<&str> {
        self.fallback.as_deref()
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
    keys: Vec<EntryKey>,
    /// The display value for this entry.
    value: String,
}

impl Entry {
    /// Creates a new entry with a given key and value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::types::rich::{Entry, EntryKey};
    ///
    /// let entry = Entry::new(1, "Full Health");
    /// assert_eq!(entry.value(), "Full Health");
    ///
    /// let range_entry = Entry::new(EntryKey::Range(1..=100), "Health Range");
    /// assert_eq!(range_entry.value(), "Health Range");
    /// ```
    pub fn new(key: impl Into<EntryKey>, value: impl Into<String>) -> Self {
        Self {
            keys: vec![key.into()],
            value: value.into(),
        }
    }

    /// Adds an alternate key to the entry, returning `self` for chaining.
    ///
    /// The keys are automatically merged with any existing keys:
    /// contiguous values are collapsed into ranges, and overlapping
    /// ranges are combined.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::types::rich::Entry;
    ///
    /// let entry = Entry::new(1, "Full Health").with_key(2).with_key(3);
    /// assert_eq!(entry.keys().len(), 1);
    /// assert_eq!(entry.to_string(), "1-3=Full Health");
    /// ```
    #[must_use]
    pub fn with_key(mut self, key: impl Into<EntryKey>) -> Entry {
        let new_key = key.into();
        self.keys = merge_key_slices(&self.keys, &[new_key]);
        self
    }

    /// Merges another entry with the same display value into this entry,
    /// combining their keys and collapsing contiguous ranges.
    ///
    /// # Panics
    ///
    /// Panics if the entries have different display values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcheevos::types::rich::Entry;
    ///
    /// let entry = Entry::new(1, "A").merge(&Entry::new(2, "A"));
    /// assert_eq!(entry.to_string(), "1-2=A");
    /// ```
    #[must_use]
    pub fn merge(mut self, other: &Entry) -> Entry {
        assert_eq!(
            self.value, other.value,
            "cannot merge entries with different values"
        );
        self.keys = merge_key_slices(&self.keys, &other.keys);
        self
    }

    /// Returns the keys for this entry.
    #[must_use]
    pub fn keys(&self) -> &[EntryKey] {
        &self.keys
    }

    /// Returns the display value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
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

impl EntryKey {
    /// Returns the minimum `u32` value covered by this key.
    #[must_use]
    fn min_value(&self) -> u32 {
        match self {
            Self::Value(v) => *v,
            Self::Range(range) => *range.start(),
        }
    }

    /// Returns the maximum `u32` value covered by this key.
    #[must_use]
    fn max_value(&self) -> u32 {
        match self {
            Self::Value(v) => *v,
            Self::Range(range) => *range.end(),
        }
    }
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

/// Merges two sorted, non-overlapping key slices into a single sorted slice
/// with contiguous runs collapsed into ranges.
fn merge_key_slices(a: &[EntryKey], b: &[EntryKey]) -> Vec<EntryKey> {
    let mut result: Vec<EntryKey> = Vec::with_capacity(a.len() + b.len());
    let mut i = 0;
    let mut j = 0;

    let mut pick_next = || match (a.get(i), b.get(j)) {
        (Some(ka), Some(kb)) if ka.min_value() <= kb.min_value() => {
            i += 1;
            Some(ka)
        }
        (Some(_) | None, Some(kb)) => {
            j += 1;
            Some(kb)
        }
        (Some(ka), None) => {
            i += 1;
            Some(ka)
        }
        (None, None) => None,
    };

    while let Some(key) = pick_next() {
        if let Some(last) = result.last_mut()
            && key.min_value() <= last.max_value() + 1
        {
            let new_end = last.max_value().max(key.max_value());
            *last = if last.min_value() == new_end {
                EntryKey::Value(new_end)
            } else {
                EntryKey::Range(last.min_value()..=new_end)
            };
            continue;
        }
        result.push(key.clone());
    }

    result
}

/// Merges entries with the same display value, collapsing contiguous key
/// ranges and sorting by minimum key value.
fn optimize_entries(entries: &[Entry]) -> Vec<Entry> {
    let mut result: Vec<Entry> = Vec::new();

    for entry in entries {
        if let Some(existing) = result.iter_mut().find(|e| e.value == entry.value) {
            existing.keys = merge_key_slices(&existing.keys, &entry.keys);
        } else {
            result.push(entry.clone());
        }
    }

    result.sort_by(|a, b| {
        let a_min = a.keys.first().map_or(0, EntryKey::min_value);
        let b_min = b.keys.first().map_or(0, EntryKey::min_value);
        a_min.cmp(&b_min)
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_table() {
        let table = LookupTable::new("T");
        assert_eq!(table.entries().count(), 0);
    }

    #[test]
    fn single_entry_unchanged() {
        let table = LookupTable::new("T").with_entry(Entry::new(1, "A"));
        let mut entries = table.entries();
        assert_eq!(entries.next().unwrap().to_string(), "1=A");
        assert!(entries.next().is_none());
    }

    #[test]
    fn same_value_contiguous_to_range() {
        let table = LookupTable::new("T").with_entries([(1, "A"), (2, "A"), (3, "A")]);
        let mut entries = table.entries();
        assert_eq!(entries.next().unwrap().to_string(), "1-3=A");
    }

    #[test]
    fn same_value_non_contiguous_list() {
        let table = LookupTable::new("T").with_entries([(1, "A"), (3, "A"), (5, "A")]);
        let mut entries = table.entries();
        assert_eq!(entries.next().unwrap().to_string(), "1,3,5=A");
    }

    #[test]
    fn partial_contiguous_ranges() {
        let table =
            LookupTable::new("T").with_entries([(1, "A"), (2, "A"), (5, "A"), (6, "A"), (7, "A")]);
        let mut entries = table.entries();
        assert_eq!(entries.next().unwrap().to_string(), "1-2,5-7=A");
    }

    #[test]
    fn different_values_sorted_by_min_key() {
        let table = LookupTable::new("T").with_entries([(5, "B"), (1, "A"), (2, "A")]);
        let mut entries = table.entries();
        assert_eq!(entries.next().unwrap().to_string(), "1-2=A");
        assert_eq!(entries.next().unwrap().to_string(), "5=B");
    }

    #[test]
    fn existing_ranges_and_values_merged() {
        let table = LookupTable::new("T")
            .with_entry(Entry::new(EntryKey::Range(1..=3), "A"))
            .with_entry(Entry::new(4, "A"));
        let mut entries = table.entries();
        assert_eq!(entries.next().unwrap().to_string(), "1-4=A");
    }

    #[test]
    fn adjacent_ranges_merged() {
        let table = LookupTable::new("T")
            .with_entry(Entry::new(EntryKey::Range(1..=3), "A"))
            .with_entry(Entry::new(EntryKey::Range(4..=6), "A"));
        let mut entries = table.entries();
        assert_eq!(entries.next().unwrap().to_string(), "1-6=A");
    }

    #[test]
    fn overlapping_ranges_merged() {
        let table = LookupTable::new("T")
            .with_entry(Entry::new(EntryKey::Range(1..=5), "A"))
            .with_entry(Entry::new(EntryKey::Range(3..=7), "A"));
        let mut entries = table.entries();
        assert_eq!(entries.next().unwrap().to_string(), "1-7=A");
    }

    #[test]
    fn with_key_builder_merged() {
        let table = LookupTable::new("T")
            .with_entry(Entry::new(1, "A").with_key(3))
            .with_entry(Entry::new(2, "A"));
        let mut entries = table.entries();
        assert_eq!(entries.next().unwrap().to_string(), "1-3=A");
    }

    #[test]
    fn different_values_no_merge() {
        let table = LookupTable::new("T")
            .with_entry(Entry::new(1, "A"))
            .with_entry(Entry::new(2, "B"));
        let mut entries = table.entries();
        assert_eq!(entries.next().unwrap().to_string(), "1=A");
        assert_eq!(entries.next().unwrap().to_string(), "2=B");
    }

    #[test]
    fn with_key_merges_contiguous_into_range() {
        let entry = Entry::new(1, "A").with_key(2).with_key(3);
        assert_eq!(entry.to_string(), "1-3=A");
    }

    #[test]
    fn with_key_non_contiguous_stays_separate() {
        let entry = Entry::new(1, "A").with_key(3).with_key(5);
        assert_eq!(entry.to_string(), "1,3,5=A");
    }

    #[test]
    fn with_key_range_merged_into_existing() {
        let entry = Entry::new(EntryKey::Range(1..=3), "A").with_key(EntryKey::Range(5..=7));
        assert_eq!(entry.to_string(), "1-3,5-7=A");
    }

    #[test]
    fn with_key_out_of_order_still_sorted() {
        let entry = Entry::new(3, "A").with_key(1).with_key(2);
        assert_eq!(entry.to_string(), "1-3=A");
    }

    #[test]
    fn with_key_adjacent_ranges_merged() {
        let entry = Entry::new(EntryKey::Range(1..=3), "A").with_key(EntryKey::Range(4..=6));
        assert_eq!(entry.to_string(), "1-6=A");
    }

    #[test]
    fn with_key_overlapping_ranges_merged() {
        let entry = Entry::new(EntryKey::Range(1..=5), "A").with_key(EntryKey::Range(3..=7));
        assert_eq!(entry.to_string(), "1-7=A");
    }

    #[test]
    fn display_output_integration() {
        let table = LookupTable::new("Test")
            .with_entry(Entry::new(1, "Level 2"))
            .with_entry(Entry::new(2, "Level 2"))
            .with_entry(Entry::new(5, "Level 1"))
            .with_entry(Entry::new(3, "Level 2"))
            .with_entry(Entry::new(4, "Level 2"));
        let output = table.to_string();
        assert_eq!(output, "Lookup:Test\n1-4=Level 2\n5=Level 1\n");
    }
}

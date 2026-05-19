//! Supported hash type for ROM identification.

use std::{fs, io, path::Path};

/// A supported ROM hash and its associated name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportedHash {
    /// The ROM hash string.
    hash: String,
    /// The ROM name.
    name: String,
}

impl SupportedHash {
    /// Creates a new supported hash.
    pub fn new(hash: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            hash: hash.into(),
            name: name.into(),
        }
    }

    /// Returns the hash string.
    #[must_use]
    pub fn hash(&self) -> &str {
        &self.hash
    }

    /// Returns the ROM name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Parses a CSV file of hashes into a vector of [`SupportedHash`] values.
    ///
    /// Each line should be in the format `hash, name`. Malformed or empty lines
    /// are skipped with a warning printed to stderr.
    pub fn parse_from_file(path: &Path) -> io::Result<Vec<Self>> {
        let content = fs::read_to_string(path)?;
        let hashes: Vec<Self> = content
            .lines()
            .enumerate()
            .filter_map(|(i, line)| {
                let (hash, name) = line.split_once(',').or_else(|| {
                    if !line.trim().is_empty() {
                        eprintln!(
                            "Warning: skipping malformed line {} in {}: {line}",
                            i + 1,
                            path.display()
                        );
                    }
                    None
                })?;
                let hash = hash.trim();
                let name = name.trim();
                if hash.is_empty() || name.is_empty() {
                    eprintln!(
                        "Warning: skipping empty field on line {} in {}: {line}",
                        i + 1,
                        path.display()
                    );
                    return None;
                }
                Some(Self::new(hash, name))
            })
            .collect();
        Ok(hashes)
    }
}

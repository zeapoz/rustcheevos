use std::{fmt, str::FromStr};

use winnow::Parser;

use crate::{ParseError, parsers::parse_hit_count};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct HitCount(u32);

impl HitCount {
    /// Creates a new hit count with the given hits.
    pub fn new(hits: u32) -> Self {
        Self(hits)
    }

    /// Sets the hits on this hit count.
    pub fn set_hits(&mut self, hits: u32) {
        self.0 = hits;
    }
}

impl FromStr for HitCount {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_hit_count
            .parse(s)
            .map_err(|s| ParseError::InvalidHitCount(s.to_string()))
    }
}

impl fmt::Display for HitCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 > 0 {
            write!(f, ".{}.", self.0)
        } else {
            Ok(())
        }
    }
}

impl From<u32> for HitCount {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

//! Definitions for user file headers.

use std::{fmt, str::FromStr};

use super::ParseError;

/// The header of a user file.
#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    /// The protocol version.
    pub version: ProtocolVersion,
    /// The game title.
    pub game_title: String,
}

impl Header {
    /// Creates a new header with the given game title.
    pub fn new(game_title: impl Into<String>) -> Self {
        Self {
            version: ProtocolVersion::V1_3,
            game_title: game_title.into(),
        }
    }
}

impl FromStr for Header {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((version, title)) = s.split_once('\n') {
            Ok(Header {
                version: version.parse()?,
                game_title: title.trim().to_string(),
            })
        } else {
            Err(ParseError::InvalidHeader("invalid header format".into()))
        }
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self.version, self.game_title)
    }
}

/// The protocol version of a user file.
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProtocolVersion {
    V1_3,
}

impl FromStr for ProtocolVersion {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1.3" => Ok(ProtocolVersion::V1_3),
            _ => Err(ParseError::InvalidProtocolVersion(s.to_string())),
        }
    }
}

impl fmt::Display for ProtocolVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProtocolVersion::V1_3 => write!(f, "1.3"),
        }
    }
}

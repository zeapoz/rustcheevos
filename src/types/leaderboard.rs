use std::fmt;
use std::str::FromStr;

use super::ParseError;
use super::achievement::ConditionGroup;

#[derive(Debug, Clone, PartialEq)]
pub enum Format {
    Score,
    Seconds,
    Frames,
    Milliseconds,
    Minutes,
    SecsAsMins,
    Value,
    Unsigned,
    Tens,
    Hundreds,
    Thousands,
    Fixed1,
    Fixed2,
    Fixed3,
    Points,
    Custom,
}

impl Format {
    pub fn as_str(&self) -> &'static str {
        match self {
            Format::Score => "SCORE",
            Format::Seconds => "SECONDS",
            Format::Frames => "FRAMES",
            Format::Milliseconds => "MILLISECONDS",
            Format::Minutes => "MINUTES",
            Format::SecsAsMins => "SECS_AS_MINS",
            Format::Value => "VALUE",
            Format::Unsigned => "UNSIGNED",
            Format::Tens => "TENS",
            Format::Hundreds => "HUNDREDS",
            Format::Thousands => "THOUSANDS",
            Format::Fixed1 => "FIXED1",
            Format::Fixed2 => "FIXED2",
            Format::Fixed3 => "FIXED3",
            Format::Points => "POINTS",
            Format::Custom => "CUSTOM",
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Format {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let format = match s.to_uppercase().as_str() {
            "SCORE" => Self::Score,
            "SECONDS" => Self::Seconds,
            "FRAMES" => Self::Frames,
            "MILLISECONDS" => Self::Milliseconds,
            "MINUTES" => Self::Minutes,
            "SECS_AS_MINS" => Self::SecsAsMins,
            "VALUE" => Self::Value,
            "UNSIGNED" => Self::Unsigned,
            "TENS" => Self::Tens,
            "HUNDREDS" => Self::Hundreds,
            "THOUSANDS" => Self::Thousands,
            "FIXED1" => Self::Fixed1,
            "FIXED2" => Self::Fixed2,
            "FIXED3" => Self::Fixed3,
            "POINTS" => Self::Points,
            "CUSTOM" => Self::Custom,
            _s => return Err(ParseError::InvalidFormat),
        };
        Ok(format)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LeaderboardConditions {
    pub start: ConditionGroup,
    pub cancel: ConditionGroup,
    pub submit: ConditionGroup,
    pub value: ConditionGroup,
}

impl LeaderboardConditions {
    pub fn new(
        start: impl Into<ConditionGroup>,
        cancel: impl Into<ConditionGroup>,
        submit: impl Into<ConditionGroup>,
        value: impl Into<ConditionGroup>,
    ) -> Self {
        Self {
            start: start.into(),
            cancel: cancel.into(),
            submit: submit.into(),
            value: value.into(),
        }
    }
}

impl FromStr for LeaderboardConditions {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<_> = s.split(':').collect();
        if fields.len() < 4 {
            return Err(ParseError::InvalidLeaderboard("insufficient fields".into()));
        }

        let start: ConditionGroup = fields[0].parse()?;
        let cancel: ConditionGroup = fields[1].parse()?;
        let submit: ConditionGroup = fields[2].parse()?;
        let value: ConditionGroup = fields[3].parse()?;

        Ok(Self {
            start,
            cancel,
            submit,
            value,
        })
    }
}

impl fmt::Display for LeaderboardConditions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}:{}",
            self.start, self.cancel, self.submit, self.value
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Leaderboard {
    pub id: Option<u32>,
    pub title: String,
    pub description: String,
    pub conditions: LeaderboardConditions,
    pub format: Format,
    pub lower_is_better: bool,
}

impl Leaderboard {
    pub fn new(
        title: impl Into<String>,
        description: impl Into<String>,
        conditions: LeaderboardConditions,
        format: Format,
        lower_is_better: bool,
    ) -> Self {
        Self {
            conditions,
            id: None,
            title: title.into(),
            description: description.into(),
            format,
            lower_is_better,
        }
    }

    pub fn with_id(mut self, id: u32) -> Self {
        self.id = Some(id);
        self
    }
}

impl fmt::Display for Leaderboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "L{}:\"{}\":\"{}\":\"{}\":\"{}\":{}:\"{}\":\"{}\":{}",
            self.id.unwrap_or(0),
            self.conditions.start,
            self.conditions.cancel,
            self.conditions.submit,
            self.conditions.value,
            self.format,
            self.title,
            self.description,
            if self.lower_is_better { 1 } else { 0 }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_format_from_str() {
        assert_eq!(Format::from_str("SCORE").unwrap(), Format::Score);
        assert_eq!(Format::from_str("seconds").unwrap(), Format::Seconds);
        assert!(Format::from_str("INVALID").is_err());
    }

    #[test]
    fn test_format_display() {
        assert_eq!(Format::Score.to_string(), "SCORE");
        assert_eq!(Format::Milliseconds.to_string(), "MILLISECONDS");
    }

    #[test]
    fn test_leaderboard_conditions_from_str() {
        let input = "0xH001234=1:0xH001235=1:0xH001236=1:0xH001236";
        let cond: LeaderboardConditions = input.parse().unwrap();

        assert_eq!(cond.start.iter().count(), 1);
        assert_eq!(cond.cancel.iter().count(), 1);
        assert_eq!(cond.submit.iter().count(), 1);
        assert_eq!(cond.value.iter().count(), 1);
    }

    #[test]
    fn test_leaderboard_conditions_display() {
        let start: ConditionGroup = "0xH001234=1".parse().unwrap();
        let cancel: ConditionGroup = "0xH001235=1".parse().unwrap();
        let submit: ConditionGroup = "0xH001236=1".parse().unwrap();
        let value: ConditionGroup = "0xH001236*10".parse().unwrap();
        let cond = LeaderboardConditions::new(start, cancel, submit, value);
        let output = cond.to_string();
        assert!(output.starts_with("0xH4d2=1:0xH4d3=1:0xH4d4=1:0xH4d4*10"));
    }

    #[test]
    fn test_leaderboard_display() {
        let start: ConditionGroup = "0xH001234=1".parse().unwrap();
        let cancel: ConditionGroup = "0xH001235=1".parse().unwrap();
        let submit: ConditionGroup = "0xH001236=1".parse().unwrap();
        let value: ConditionGroup = "0xH001236".parse().unwrap();
        let cond = LeaderboardConditions::new(start, cancel, submit, value);
        let lb = Leaderboard::new(
            "Test Leaderboard",
            "Test Description",
            cond,
            Format::Score,
            false,
        );
        let output = format!("{}", lb);
        assert!(output.starts_with("L0:"));
        assert!(output.contains("Test Leaderboard"));
        assert!(output.contains("SCORE"));
    }

    #[test]
    fn test_leaderboard_with_id() {
        let start: ConditionGroup = "0xH001234=1".parse().unwrap();
        let cancel: ConditionGroup = "0xH001235=1".parse().unwrap();
        let submit: ConditionGroup = "0xH001236=1".parse().unwrap();
        let value: ConditionGroup = "0xH001236".parse().unwrap();
        let cond = LeaderboardConditions::new(start, cancel, submit, value);
        let lb = Leaderboard::new("Test", "Desc", cond, Format::Seconds, true).with_id(12345);

        let output = lb.to_string();
        assert!(output.starts_with("L12345:"));
    }
}

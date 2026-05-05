use std::fmt;
use std::str::FromStr;

use super::ParseError;
use super::condition::Condition;

#[derive(Debug, Clone, PartialEq)]
pub struct ConditionGroup(Vec<Condition>);

impl ConditionGroup {
    pub fn new(conditions: Vec<Condition>) -> Self {
        Self(conditions)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Condition> {
        self.0.iter()
    }

    pub fn into_inner(self) -> Vec<Condition> {
        self.0
    }
}

impl From<Condition> for ConditionGroup {
    fn from(value: Condition) -> Self {
        ConditionGroup::new(vec![value])
    }
}

impl<const N: usize> From<[Condition; N]> for ConditionGroup {
    fn from(arr: [Condition; N]) -> Self {
        ConditionGroup::new(arr.into())
    }
}

impl From<Vec<Condition>> for ConditionGroup {
    fn from(value: Vec<Condition>) -> Self {
        ConditionGroup::new(value)
    }
}

impl FromStr for ConditionGroup {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let conditions: Vec<_> = s
            .split('_')
            .filter(|s| !s.is_empty())
            .map(Condition::deserialize)
            .collect::<Result<_, _>>()?;

        Ok(Self(conditions))
    }
}

impl fmt::Display for ConditionGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join("_")
        )
    }
}

pub fn extend_from_item(vec: &mut Vec<Condition>, item: impl Into<ConditionGroup>) {
    vec.extend(item.into().0);
}

pub struct AltGroups(pub Vec<ConditionGroup>);

impl From<Vec<ConditionGroup>> for AltGroups {
    fn from(v: Vec<ConditionGroup>) -> Self {
        AltGroups(v)
    }
}

impl From<Vec<Condition>> for AltGroups {
    fn from(v: Vec<Condition>) -> Self {
        AltGroups(v.into_iter().map(ConditionGroup::from).collect())
    }
}

impl<const N: usize> From<[Condition; N]> for AltGroups {
    fn from(arr: [Condition; N]) -> Self {
        AltGroups(arr.into_iter().map(ConditionGroup::from).collect())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Conditions {
    pub core: ConditionGroup,
    pub alt_groups: Vec<ConditionGroup>,
}

impl Conditions {
    pub fn new<C: Into<ConditionGroup>>(core: C) -> Self {
        Self {
            core: core.into(),
            alt_groups: Vec::new(),
        }
    }

    pub fn with_alts<I: IntoIterator<Item = ConditionGroup>>(mut self, alts: I) -> Self {
        self.alt_groups = alts.into_iter().collect();
        self
    }
}

impl std::str::FromStr for Conditions {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let groups: Vec<_> = s.split('S').filter(|s| !s.is_empty()).collect();
        let core: ConditionGroup = groups.first().ok_or(ParseError::InvalidFormat)?.parse()?;

        let alt_groups = groups
            .iter()
            .skip(1)
            .map(|s| s.parse())
            .collect::<Result<_, _>>()?;

        Ok(Self { core, alt_groups })
    }
}

impl fmt::Display for Conditions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.core)?;
        for alt in &self.alt_groups {
            write!(f, "S{alt}")?;
        }
        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum Tag {
    #[default]
    Empty,
    Progression,
    WinCondition,
    Missable,
}

impl Tag {
    pub fn as_str(&self) -> &'static str {
        match self {
            Tag::Empty => "",
            Tag::Progression => "progression",
            Tag::WinCondition => "win_condition",
            Tag::Missable => "missable",
        }
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for Tag {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tag = match s {
            "" | "empty" => Self::Empty,
            "progression" => Self::Progression,
            "win_condition" => Self::WinCondition,
            "missable" => Self::Missable,
            s => return Err(ParseError::InvalidTag(s.to_string())),
        };
        Ok(tag)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Achievement {
    pub id: Option<u32>,
    pub title: String,
    pub description: String,
    pub conditions: Conditions,
    pub tag: Tag,
    pub points: u32,
}

impl Achievement {
    pub fn new<S: Into<String>>(
        title: S,
        description: S,
        conditions: Conditions,
        points: u32,
    ) -> Self {
        Self {
            conditions,
            id: None,
            title: title.into(),
            description: description.into(),
            tag: Tag::default(),
            points,
        }
    }

    pub fn with_id(mut self, id: u32) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_tag(mut self, tag: Tag) -> Self {
        self.tag = tag;
        self
    }
}

impl fmt::Display for Achievement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.conditions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_deserialize_condition_groups_no_alts() {
        let input =
            "I:0xH1a8c94*184_0x 1aaee4=5_A:1_I:0xH1a8c94*184_d0x 1aaee4=0x 1aaee4_0xH1a54c0=3";

        let groups = Conditions::from_str(input).expect("Failed to parse condition groups");

        assert_eq!(groups.core.iter().count(), 6);
        assert!(groups.alt_groups.is_empty());
    }

    #[test]
    fn test_deserialize_condition_groups_alt_groups() {
        let input = "I:0xH1a8c94*2_0xU1a9c4d>=1_I:0xH1a8c94*2_0xU1a9c4d!=4_I:0xH1a8c94*2_0xU1a9c7d>=1_I:0xH1a8c94*2_0xU1a9c7d!=4_I:0xH1a8c94*2_0xU1a9cad>=1_I:0xH1a8c94*2_0xU1a9cad!=4_d0xH1a6990=1_0xH1a6990=1SI:0xH1a8c94*2_d0xU1a9c4d<1SI:0xH1a8c94*2_d0xU1a9c7d<1SI:0xH1a8c94*2_d0xU1a9cad<1";

        let groups = Conditions::from_str(input).expect("Failed to parse condition groups");

        assert_eq!(groups.core.iter().count(), 14);
        assert_eq!(groups.alt_groups.len(), 3);

        for alt in &groups.alt_groups {
            assert_eq!(alt.iter().count(), 2);
        }
    }

    #[test]
    fn test_condition_groups_display() {
        let input = "I:0xH1a8c94*2_0xU1a9fad>=2";
        let groups: Conditions = input.parse().unwrap();
        let output = format!("{}", groups);
        assert!(output.starts_with("I:0xH1a8c94*2"));
    }

    #[test]
    fn test_achievement_display() {
        let input = "I:0xH1a8c94*2_0xU1a9fad>=2";
        let groups: Conditions = input.parse().unwrap();
        let achievement = Achievement::new(
            String::from("Test Title"),
            String::from("Test Description"),
            groups,
            5,
        );
        let output = format!("{}", achievement);
        assert!(output.starts_with("I:0xH1a8c94*2"));
    }
}

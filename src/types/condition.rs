#![allow(clippy::should_implement_trait)]

use crate::types::ParseError;
use crate::types::memory::MemoryRef;
use crate::types::memory::{MemOrValue, MemorySize, MemoryType};
use regex::Regex;
use std::fmt;
use std::str::FromStr;
use std::sync::LazyLock;

use super::flag::Flag;
use super::operator::Operator;
use super::source::{Operation, Source};

macro_rules! condition_op_method {
    ($name:ident, $op:expr) => {
        pub fn $name<T: Into<MemOrValue>>(mut self, other: T) -> Self {
            self.op = Some(Operation {
                op: $op,
                target: other.into(),
            });
            self
        }
    };
}

macro_rules! condition_flag_method {
    ($name:ident, $flag:expr) => {
        pub fn $name(mut self) -> Self {
            self.source.flag = Some($flag);
            self
        }
    };
}

macro_rules! condition_memtype_method {
    ($name:ident, $memtype:expr) => {
        pub fn $name(mut self) -> Self {
            self.source.memtype = Some($memtype);
            self
        }
    };
}

pub trait WithFlagExt {
    fn with_flag(self, flag: Flag) -> Self;
}

impl<const N: usize> WithFlagExt for [Condition; N] {
    fn with_flag(mut self, flag: Flag) -> Self {
        for item in self.iter_mut() {
            item.source.flag = Some(flag);
        }
        self
    }
}

impl WithFlagExt for Vec<Condition> {
    fn with_flag(mut self, flag: Flag) -> Self {
        for cond in self.iter_mut() {
            cond.source.flag = Some(flag);
        }
        self
    }
}

static CONDITION_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?x)
        (?<flag>[A-Z]:)?
        (?<memtype>[dpb~])?
        (?<memsize>0x[A-Z]|f[FBHIML])?
        (?<value>[0-9a-fA-F]+)?
        (?<operator>(!=|>=|<=|[*/+-=><!]))?
        (?<memsize2>0x[A-Z]|f[FBHIML])?
        (?<value2>[0-9a-fA-F]+)?
        \.?(?<hits>\d)?\.?
        ",
    )
    .expect("regex must be valid")
});

fn parse_value(s: &str) -> Result<u32, ParseError> {
    s.parse()
        .or_else(|_| u32::from_str_radix(s, 16))
        .map_err(|_| ParseError::InvalidValue)
}

fn build_memref(
    value: u32,
    memsize: Option<MemorySize>,
    memtype: Option<MemoryType>,
) -> MemOrValue {
    match memsize {
        Some(size) => MemOrValue::Memory(MemoryRef {
            size,
            address: value as usize,
            memtype: memtype.unwrap_or_default(),
        }),
        None => MemOrValue::Value { value },
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConditionGroup(Vec<Condition>);

impl ConditionGroup {
    pub fn new(conditions: Vec<Condition>) -> Self {
        Self(conditions)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Condition> {
        self.0.iter()
    }

    pub fn and(mut self, other: impl Into<ConditionGroup>) -> Self {
        self.0.extend(other.into().0);
        self
    }

    pub fn with_flag(mut self, flag: Flag) -> ConditionGroup {
        for cond in self.0.iter_mut() {
            cond.source.flag = Some(flag);
        }
        self
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

#[derive(Clone, Debug, PartialEq)]
pub struct Condition {
    pub source: Source,
    pub op: Option<Operation>,
    pub hits: u32,
}

impl Condition {
    pub fn with_memtype(mut self, memtype: MemoryType) -> Self {
        self.source.memtype = Some(memtype);
        self
    }

    pub fn with_hits(mut self, hits: u32) -> Self {
        self.hits = hits;
        self
    }

    condition_op_method!(eq, Operator::Equals);
    condition_op_method!(neq, Operator::NotEquals);
    condition_op_method!(gt, Operator::GreaterThan);
    condition_op_method!(gte, Operator::GreaterThanOrEquals);
    condition_op_method!(lt, Operator::LessThan);
    condition_op_method!(lte, Operator::LessThanOrEquals);
    condition_op_method!(add, Operator::Add);
    condition_op_method!(sub, Operator::Subtract);
    condition_op_method!(mul, Operator::Multiply);
    condition_op_method!(div, Operator::Divide);

    condition_flag_method!(pause_if, Flag::PauseIf);
    condition_flag_method!(reset_if, Flag::ResetIf);
    condition_flag_method!(reset_next_if, Flag::ResetNextIf);
    condition_flag_method!(add_source, Flag::AddSource);
    condition_flag_method!(sub_source, Flag::SubSource);
    condition_flag_method!(add_hits, Flag::AddHits);
    condition_flag_method!(sub_hits, Flag::SubHits);
    condition_flag_method!(add_address, Flag::AddAddress);
    condition_flag_method!(and_next, Flag::AndNext);
    condition_flag_method!(or_next, Flag::OrNext);
    condition_flag_method!(measured, Flag::Measured);
    condition_flag_method!(measured_pct, Flag::MeasuredPercentage);
    condition_flag_method!(measured_if, Flag::MeasuredIf);
    condition_flag_method!(trigger, Flag::Trigger);
    condition_flag_method!(remember, Flag::Remember);

    condition_memtype_method!(delta, MemoryType::Delta);
    condition_memtype_method!(prior, MemoryType::Prior);
    condition_memtype_method!(bcd, MemoryType::BCD);
    condition_memtype_method!(invert, MemoryType::Invert);

    pub fn deserialize(s: &str) -> Result<Self, ParseError> {
        let Some(caps) = CONDITION_REGEX.captures(s) else {
            return Err(ParseError::InvalidFormat);
        };

        let flag = caps
            .name("flag")
            .and_then(|s| Flag::from_str(s.as_str()).ok());

        let memtype = caps
            .name("memtype")
            .and_then(|s| MemoryType::from_str(s.as_str()).ok());

        let memsize = caps
            .name("memsize")
            .and_then(|s| MemorySize::from_str(s.as_str()).ok());

        let memsize2 = caps
            .name("memsize2")
            .and_then(|s| MemorySize::from_str(s.as_str()).ok());

        let value = caps
            .name("value")
            .ok_or(ParseError::EmptyInput)
            .and_then(|s| parse_value(s.as_str()))?;

        let operator = caps
            .name("operator")
            .and_then(|s| Operator::from_str(s.as_str()).ok());

        let hits: u32 = caps
            .name("hits")
            .and_then(|s| s.as_str().parse().ok())
            .unwrap_or(0);

        let target: Option<u32> = caps
            .name("value2")
            .map(|s| parse_value(s.as_str()))
            .transpose()?;

        let reference = build_memref(value, memsize, memtype);

        let target = target.map(|t| build_memref(t, memsize2, None));

        let op = operator
            .map(|op| -> Result<Operation, ParseError> {
                let target = target.ok_or(ParseError::OperatorRequiresTarget)?;
                Ok(Operation { op, target })
            })
            .transpose()?;

        Ok(Self {
            source: Source {
                reference,
                flag,
                memtype,
            },
            op,
            hits,
        })
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(flag) = &self.source.flag {
            write!(f, "{}", flag.to_prefix())?;
        }
        if let Some(memtype) = &self.source.memtype {
            write!(f, "{}", memtype.to_prefix())?;
        }
        write!(f, "{}", self.source.reference)?;
        if let Some(op) = &self.op {
            write!(f, "{}", op.op.to_prefix())?;
            write!(f, "{}", op.target)?;
        }
        if self.hits > 0 {
            write!(f, ".{}.", self.hits)?;
        }
        Ok(())
    }
}

impl WithFlagExt for Condition {
    fn with_flag(mut self, flag: Flag) -> Self {
        self.source.flag = Some(flag);
        self
    }
}

pub fn extend_from_item(vec: &mut Vec<Condition>, item: impl Into<ConditionGroup>) {
    vec.extend(item.into().0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::achievement::Conditions;
    use std::str::FromStr;

    #[test]
    fn test_deserialize_condition_group() {
        let input = "A:1_M:d0xX31d05c=0xX31d05c.3._Q:d0xH1a6990=1_Q:0xH1a6990=1_R:0xH1a6990!=1";

        let group = ConditionGroup::from_str(input).expect("Failed to parse");

        assert_eq!(group.iter().count(), 5);
    }

    #[test]
    fn test_condition_group_display() {
        let input = "M:0xX31d05c=1_A:1";
        let group: ConditionGroup = input.parse().unwrap();
        let output = format!("{}", group);
        assert!(output.starts_with("M:0xX31d05c=1_"));
        assert!(output.contains("_A:1"));
    }

    #[test]
    fn test_condition_groups_display_no_alts() {
        let input = "I:0xH1a8c94*2_0xU1a9fad>=2";
        let groups: Conditions = input.parse().unwrap();
        let output = format!("{}", groups);
        assert!(output.starts_with("I:0xH1a8c94*2"));
        assert!(output.contains("_0xU1a9fad>=2"));
    }

    #[test]
    fn test_condition_groups_display_with_alts() {
        let input = "I:0xH1a8c94*2_0xU1a9fad>=2SI:0xH1a8c94*2_d0xU1a9fad<2";
        let groups: Conditions = input.parse().unwrap();
        let output = format!("{}", groups);
        assert!(output.starts_with("I:0xH1a8c94*2"));
        assert!(output.contains("SI:"));
    }

    #[test]
    fn test_delta_serialization() {
        let cond = Condition {
            source: Source {
                reference: MemOrValue::Memory(MemoryRef {
                    size: MemorySize::Bits32,
                    address: 0x31d05c,
                    memtype: MemoryType::Delta,
                }),
                flag: None,
                memtype: None,
            },
            op: Some(Operation {
                op: Operator::Equals,
                target: MemOrValue::Memory(MemoryRef {
                    size: MemorySize::Bits32,
                    address: 0x31d05c,
                    memtype: MemoryType::Standard,
                }),
            }),
            hits: 0,
        };
        assert_eq!(cond.to_string(), "d0xX31d05c=0xX31d05c");
    }
}

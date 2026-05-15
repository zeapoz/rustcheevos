//! Parser functions for requirements.

use winnow::{
    Parser, Result,
    combinator::{alt, opt},
};

use crate::{
    prelude::{Arithmetic, Condition, Requirement},
    types::requirement::{
        arithmetic::ArithmeticOperation,
        condition::{ComparisonOperation, hits::HitCount},
    },
};

use super::{
    parse_arithmetic_flag, parse_arithmetic_operator, parse_comparison_flag,
    parse_comparison_operator, parse_int_value, parse_typed_value,
};

/// Parses a requirement.
pub fn parse_requirement(input: &mut &str) -> Result<Requirement> {
    let requirement = alt((
        parse_arithmetic.map(Requirement::from),
        parse_condition.map(Requirement::from),
    ))
    .parse_next(input)?;
    Ok(requirement)
}

/// Parses a comparison condition.
pub fn parse_condition(input: &mut &str) -> Result<Condition> {
    let flag = opt(parse_comparison_flag).parse_next(input)?;
    let lhs = parse_typed_value.parse_next(input)?;
    let operation = parse_comparison_operation.parse_next(input)?;
    let hit_count = opt(parse_hit_count).parse_next(input)?.unwrap_or_default();
    Ok(Condition {
        flag,
        lhs,
        operation,
        hit_count,
    })
}

/// Parses a comparison operation.
fn parse_comparison_operation(input: &mut &str) -> Result<ComparisonOperation> {
    let operator = parse_comparison_operator.parse_next(input)?;
    let rhs = parse_typed_value.parse_next(input)?;
    Ok(ComparisonOperation { operator, rhs })
}

/// Parses a hit count.
pub fn parse_hit_count(input: &mut &str) -> Result<HitCount> {
    let (_open, hit_count, _close) =
        ('.', parse_int_value.map(HitCount::from), '.').parse_next(input)?;
    Ok(hit_count)
}

/// Parses an arithmetic condition.
pub fn parse_arithmetic(input: &mut &str) -> Result<Arithmetic> {
    let flag = parse_arithmetic_flag.parse_next(input)?;
    let lhs = parse_typed_value.parse_next(input)?;
    let operation = opt(parse_arithmetic_operation).parse_next(input)?;
    Ok(Arithmetic {
        flag,
        lhs,
        operation,
    })
}

/// Parses an arithmetic operation.
fn parse_arithmetic_operation(input: &mut &str) -> Result<ArithmeticOperation> {
    let operator = parse_arithmetic_operator.parse_next(input)?;
    let rhs = parse_typed_value.parse_next(input)?;
    Ok(ArithmeticOperation { operator, rhs })
}

#[cfg(test)]
mod tests {
    use crate::types::{
        flag::ArithmeticFlag,
        memory::{MemoryRef, MemorySize},
        operator::{ArithmeticOperator, ComparisonOperator},
        requirement::condition::{ComparisonOperation, Condition, hits::HitCount},
        value::TypedValue,
    };

    use super::*;

    #[test]
    fn test_parse_valid_comparison_condition() {
        let input = "0xX1234=0xX5678";
        let requirement = input.parse::<Requirement>().unwrap();
        assert_eq!(
            requirement,
            Requirement::Condition(Condition {
                flag: None,
                lhs: TypedValue::Memory(MemoryRef::new(MemorySize::Bits32, 0x1234)),
                operation: ComparisonOperation {
                    operator: ComparisonOperator::Equals,
                    rhs: TypedValue::Memory(MemoryRef::new(MemorySize::Bits32, 0x5678))
                },
                hit_count: HitCount::new(0)
            })
        );
    }

    #[test]
    fn test_parse_valid_arithmetic_condition_in_requirement() {
        let input = "A:0xX1234+0xX5678";
        let requirement = input.parse::<Requirement>().unwrap();
        assert_eq!(
            requirement,
            Requirement::Arithmetic(Arithmetic {
                flag: ArithmeticFlag::AddSource,
                lhs: TypedValue::Memory(MemoryRef::new(MemorySize::Bits32, 0x1234)),
                operation: Some(ArithmeticOperation {
                    operator: ArithmeticOperator::Add,
                    rhs: TypedValue::Memory(MemoryRef::new(MemorySize::Bits32, 0x5678))
                })
            })
        );
    }

    #[test]
    fn test_parse_valid_hit_count() {
        let mut input = ".10.";
        let hit_count = parse_hit_count(&mut input).unwrap();
        assert_eq!(hit_count, HitCount::new(10));
    }

    #[test]
    fn test_parse_no_hit_count() {
        let mut input = "";
        let hit_count = parse_hit_count(&mut input);
        assert!(hit_count.is_err());
    }

    #[test]
    fn test_parse_invalid_hit_count() {
        let mut input = ".1a.";
        let hit_count = parse_hit_count(&mut input);
        assert!(hit_count.is_err());
    }

    #[test]
    fn test_parse_valid_comparison_condition_direct() {
        let input = "0xX1234=0xX5678";
        let condition = input.parse::<Condition>().unwrap();
        assert_eq!(condition.flag, None);
        assert_eq!(
            condition.lhs,
            TypedValue::Memory(MemoryRef::new(MemorySize::Bits32, 0x1234))
        );
        assert_eq!(condition.operation.operator, ComparisonOperator::Equals);
        assert_eq!(
            condition.operation.rhs,
            TypedValue::Memory(MemoryRef::new(MemorySize::Bits32, 0x5678))
        );
        assert_eq!(condition.hit_count, HitCount::new(0));
    }

    #[test]
    fn test_parse_arithmetic_flag() {
        let input = "A:0xX1234";
        let condition = input.parse::<Condition>();
        assert!(condition.is_err());
    }

    #[test]
    fn test_parse_valid_arithmetic_condition() {
        let input = "A:0xX1234";
        let arithmetic = input.parse::<Arithmetic>().unwrap();
        assert_eq!(arithmetic.flag, ArithmeticFlag::AddSource);
        assert_eq!(
            arithmetic.lhs,
            TypedValue::Memory(MemoryRef::new(MemorySize::Bits32, 0x1234))
        );
        assert_eq!(arithmetic.operation, None);
    }

    #[test]
    fn test_parse_valid_arithmetic_condition_with_operation() {
        let input = "A:0xX1234+0xX5678";
        let arithmetic = input.parse::<Arithmetic>().unwrap();
        assert_eq!(arithmetic.flag, ArithmeticFlag::AddSource);
        assert_eq!(
            arithmetic.lhs,
            TypedValue::Memory(MemoryRef::new(MemorySize::Bits32, 0x1234))
        );
        assert_eq!(
            arithmetic.operation,
            Some(ArithmeticOperation {
                operator: ArithmeticOperator::Add,
                rhs: TypedValue::Memory(MemoryRef::new(MemorySize::Bits32, 0x5678))
            })
        );
    }

    #[test]
    fn test_parse_valid_arithmetic_condition_with_measured_flag() {
        let input = "M:0xX1234+0xX5678";
        let arithmetic = input.parse::<Arithmetic>().unwrap();
        assert_eq!(arithmetic.flag, ArithmeticFlag::Measured);
        assert_eq!(
            arithmetic.lhs,
            TypedValue::Memory(MemoryRef::new(MemorySize::Bits32, 0x1234))
        );
        assert_eq!(
            arithmetic.operation,
            Some(ArithmeticOperation {
                operator: ArithmeticOperator::Add,
                rhs: TypedValue::Memory(MemoryRef::new(MemorySize::Bits32, 0x5678))
            })
        );
    }
}

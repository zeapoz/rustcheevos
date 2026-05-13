use winnow::{
    Parser, Result,
    combinator::{alt, opt},
};

use crate::types::requirement::comparison::*;
use crate::types::requirement::*;
use crate::types::requirement::{arithmetic::*, comparison::hits::HitCount};

use super::{
    parse_arithmetic_flag, parse_arithmetic_operator, parse_comparison_flag,
    parse_comparison_operator, parse_int_value, parse_typed_value,
};

pub fn parse_requirement(input: &mut &str) -> Result<Requirement> {
    let requirement = alt((
        parse_arithmetic_requirement.map(Requirement::from),
        parse_comparison_requirement.map(Requirement::from),
    ))
    .parse_next(input)?;
    Ok(requirement)
}

pub fn parse_comparison_requirement(input: &mut &str) -> Result<ComparisonRequirement> {
    let flag = opt(parse_comparison_flag).parse_next(input)?;
    let lhs = parse_typed_value.parse_next(input)?;
    let operation = parse_comparison_operation.parse_next(input)?;
    let hit_count = opt(parse_hit_count).parse_next(input)?.unwrap_or_default();
    Ok(ComparisonRequirement {
        flag,
        lhs,
        operation,
        hit_count,
    })
}

fn parse_comparison_operation(input: &mut &str) -> Result<ComparisonOperation> {
    let comparator = parse_comparison_operator.parse_next(input)?;
    let rhs = parse_typed_value.parse_next(input)?;
    Ok(ComparisonOperation { comparator, rhs })
}

pub fn parse_hit_count(input: &mut &str) -> Result<HitCount> {
    let (_open, hit_count, _close) =
        ('.', parse_int_value.map(HitCount::from), '.').parse_next(input)?;
    Ok(hit_count)
}

pub fn parse_arithmetic_requirement(input: &mut &str) -> Result<ArithmeticRequirement> {
    let flag = parse_arithmetic_flag.parse_next(input)?;
    let lhs = parse_typed_value.parse_next(input)?;
    let operation = opt(parse_arithmetic_operation).parse_next(input)?;
    Ok(ArithmeticRequirement {
        flag,
        lhs,
        operation,
    })
}

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
        value::TypedValue,
    };

    use super::*;

    #[test]
    fn test_parse_valid_comparison_condition() {
        let input = "0xX1234=0xX5678";
        let requirement = input.parse::<Requirement>().unwrap();
        assert_eq!(
            requirement,
            Requirement::Comparison(ComparisonRequirement {
                flag: None,
                lhs: TypedValue::Memory(MemoryRef::new(MemorySize::Bits32, 0x1234)),
                operation: ComparisonOperation {
                    comparator: ComparisonOperator::Equals,
                    rhs: TypedValue::Memory(MemoryRef::new(MemorySize::Bits32, 0x5678))
                },
                hit_count: HitCount::new(0)
            })
        );
    }

    #[test]
    fn test_parse_valid_arithmetic_condition() {
        let input = "A:0xX1234+0xX5678";
        let requirement = input.parse::<Requirement>().unwrap();
        assert_eq!(
            requirement,
            Requirement::Arithmetic(ArithmeticRequirement {
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
    fn test_parse_valid_comparison_requirement() {
        let input = "0xX1234=0xX5678";
        let requirement = input.parse::<ComparisonRequirement>().unwrap();
        assert_eq!(requirement.flag, None);
        assert_eq!(
            requirement.lhs,
            TypedValue::Memory(MemoryRef::new(MemorySize::Bits32, 0x1234))
        );
        assert_eq!(requirement.operation.comparator, ComparisonOperator::Equals);
        assert_eq!(
            requirement.operation.rhs,
            TypedValue::Memory(MemoryRef::new(MemorySize::Bits32, 0x5678))
        );
        assert_eq!(requirement.hit_count, HitCount::new(0));
    }

    #[test]
    fn test_parse_arithmetic_flag() {
        let input = "A:0xX1234";
        let requirement = input.parse::<ComparisonRequirement>();
        assert!(requirement.is_err());
    }

    #[test]
    fn test_parse_valid_arithmetic_requirement() {
        let input = "A:0xX1234";
        let requirement = input.parse::<ArithmeticRequirement>().unwrap();
        assert_eq!(requirement.flag, ArithmeticFlag::AddSource);
        assert_eq!(
            requirement.lhs,
            TypedValue::Memory(MemoryRef::new(MemorySize::Bits32, 0x1234))
        );
        assert_eq!(requirement.operation, None);
    }

    #[test]
    fn test_parse_valid_arithmetic_requirement_with_operation() {
        let input = "A:0xX1234+0xX5678";
        let requirement = input.parse::<ArithmeticRequirement>().unwrap();
        assert_eq!(requirement.flag, ArithmeticFlag::AddSource);
        assert_eq!(
            requirement.lhs,
            TypedValue::Memory(MemoryRef::new(MemorySize::Bits32, 0x1234))
        );
        assert_eq!(
            requirement.operation,
            Some(ArithmeticOperation {
                operator: ArithmeticOperator::Add,
                rhs: TypedValue::Memory(MemoryRef::new(MemorySize::Bits32, 0x5678))
            })
        );
    }

    #[test]
    fn test_parse_valid_arithmetic_requirement_with_measured_flag() {
        let input = "M:0xX1234+0xX5678";
        let requirement = input.parse::<ArithmeticRequirement>().unwrap();
        assert_eq!(requirement.flag, ArithmeticFlag::Measured);
        assert_eq!(
            requirement.lhs,
            TypedValue::Memory(MemoryRef::new(MemorySize::Bits32, 0x1234))
        );
        assert_eq!(
            requirement.operation,
            Some(ArithmeticOperation {
                operator: ArithmeticOperator::Add,
                rhs: TypedValue::Memory(MemoryRef::new(MemorySize::Bits32, 0x5678))
            })
        );
    }
}

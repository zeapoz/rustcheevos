use winnow::{Parser, Result, ascii::digit1, combinator::alt, token::take_while};

use crate::types::value::*;

use super::parse_memory_ref;

pub fn parse_value_type(input: &mut &str) -> Result<ValueType> {
    let value_types = alt(("d", "p", "b", "f", "~", "{recall}"));
    value_types
        .try_map(|c| ValueType::try_from(c))
        .parse_next(input)
}

pub fn parse_int_value(input: &mut &str) -> Result<u32> {
    digit1.parse_to().parse_next(input)
}

fn parse_float_value(input: &mut &str) -> Result<f32> {
    take_while(1.., (('0'..='9'), ('.')))
        .parse_to()
        .parse_next(input)
}

pub fn parse_value(input: &mut &str) -> Result<TypedValue> {
    if let Ok(value_type) = parse_value_type.parse_next(input) {
        return match value_type {
            ValueType::Float => Ok(TypedValue::Float(parse_float_value.parse_next(input)?)),
            ValueType::Delta => Ok(TypedValue::Delta(parse_memory_ref.parse_next(input)?)),
            ValueType::Prior => Ok(TypedValue::Prior(parse_memory_ref.parse_next(input)?)),
            ValueType::BCD => Ok(TypedValue::BCD(parse_memory_ref.parse_next(input)?)),
            ValueType::Invert => Ok(TypedValue::Invert(parse_memory_ref.parse_next(input)?)),
            ValueType::Recall => Ok(TypedValue::Recall),
            ValueType::Memory => Ok(TypedValue::Memory(parse_memory_ref.parse_next(input)?)),
            ValueType::Value => Ok(TypedValue::Value(parse_int_value.parse_next(input)?)),
        };
    }

    if let Ok(memory_ref) = parse_memory_ref.parse_next(input) {
        return Ok(TypedValue::Memory(memory_ref));
    }

    Ok(TypedValue::Value(parse_int_value.parse_next(input)?))
}

#[cfg(test)]
mod tests {
    use crate::types::memory::*;

    use super::*;

    #[test]
    fn test_parse_valid_value_type() {
        let input = "d";
        let value_type = input.parse::<ValueType>().unwrap();
        assert_eq!(value_type, ValueType::Delta);
    }

    #[test]
    fn test_parse_no_value_type() {
        let input = "";
        let value_type_or_err = input.parse::<ValueType>();
        assert!(value_type_or_err.is_err());
    }

    #[test]
    fn test_parse_invalid_value_type() {
        let input = "e";
        let value_type_or_err = input.parse::<ValueType>();
        assert!(value_type_or_err.is_err());
    }

    #[test]
    fn test_parse_valid_addr() {
        let input = "0xH1234";
        let value = input.parse::<TypedValue>().unwrap();
        assert_eq!(
            value,
            TypedValue::Memory(MemoryRef::new(MemorySize::Bits8, 0x1234))
        );
    }

    #[test]
    fn test_parse_valid_int_value() {
        let input = "1234";
        let value = input.parse::<TypedValue>().unwrap();
        assert_eq!(value, TypedValue::Value(1234));
    }

    #[test]
    fn test_parse_valid_float_value() {
        let input = "f1234.5678";
        let value = input.parse::<TypedValue>().unwrap();
        assert_eq!(value, TypedValue::Float(1234.5678));
    }

    #[test]
    fn test_parse_valid_delta() {
        let input = "d0xH1234";
        let value = input.parse::<TypedValue>().unwrap();
        assert_eq!(
            value,
            TypedValue::Delta(MemoryRef::new(MemorySize::Bits8, 0x1234))
        );
    }
}

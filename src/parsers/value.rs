use winnow::{Parser, Result, ascii::digit1, stream::Stream, token::take_while};

use crate::types::value::*;

use super::parse_memory_ref;

pub fn parse_int_value(input: &mut &str) -> Result<u32> {
    digit1.parse_to().parse_next(input)
}

fn parse_float_value(input: &mut &str) -> Result<f32> {
    let _prefix = 'f'.parse_next(input)?;
    take_while(1.., (('0'..='9'), ('.')))
        .parse_to()
        .parse_next(input)
}

pub fn parse_typed_value(input: &mut &str) -> Result<TypedValue> {
    let start = input.checkpoint();
    if let Ok(memory_ref) = parse_memory_ref.parse_next(input) {
        return Ok(TypedValue::Memory(memory_ref));
    }

    input.reset(&start);
    if let Ok(float_value) = parse_float_value.parse_next(input) {
        return Ok(TypedValue::Float(float_value));
    }

    input.reset(&start);
    Ok(TypedValue::Integer(parse_int_value.parse_next(input)?))
}

#[cfg(test)]
mod tests {
    use crate::types::memory::*;

    use super::*;

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
        assert_eq!(value, TypedValue::Integer(1234));
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
            TypedValue::Memory(MemoryRef::new(MemorySize::Bits8, 0x1234).delta())
        );
    }
}

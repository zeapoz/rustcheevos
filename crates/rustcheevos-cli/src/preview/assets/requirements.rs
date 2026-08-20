//! Requirements preview.

use std::fmt;

use rustcheevos::types::{
    chain::Chain, flag::Flag, operator::Operator, requirement::Requirement,
    requirements::Requirements, value::TypedValue,
};

use crate::preview::table::Table;

/// Column header for the row identifier.
const HEADER_ID: &str = "ID";
/// Column header for the requirement flag.
const HEADER_FLAG: &str = "Flag";
/// Column header for the value type.
const HEADER_TYPE: &str = "Type";
/// Column header for the memory size.
const HEADER_SIZE: &str = "Size";
/// Column header for the requirement operator.
const HEADER_CMP: &str = "Cmp";
/// Column header for the value/address.
const HEADER_MEM_VAL: &str = "Mem/Val";
/// Column header for the requirement hit count.
const HEADER_HITS: &str = "Hits";

/// A table-formatted group of requirements.
#[derive(Debug, Clone)]
pub struct RequirementsPreview<'a>(pub &'a [Chain]);

impl fmt::Display for RequirementsPreview<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, chain) in self.0.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            let table = chain.iter().enumerate().fold(
                Table::new([
                    HEADER_ID,
                    HEADER_FLAG,
                    HEADER_TYPE,
                    HEADER_SIZE,
                    HEADER_MEM_VAL,
                    HEADER_CMP,
                    HEADER_TYPE,
                    HEADER_SIZE,
                    HEADER_MEM_VAL,
                    HEADER_HITS,
                ]),
                |table, (idx, req)| {
                    let mut cells = requirement_row(req).to_vec();
                    cells.insert(0, (idx + 1).to_string());
                    table.row(cells)
                },
            );
            write!(f, "{table}")?;
        }
        Ok(())
    }
}

/// Builds the nine-cell row representation of a requirement.
fn requirement_row(req: &Requirement) -> [String; 9] {
    let flag = req.flag().map_or_else(String::new, flag_cell);
    let (lhs_type, lhs_size, lhs_val) = value_cells(req.lhs());
    let operator = req.operator().map_or_else(String::new, operator_cell);
    let (rhs_type, rhs_size, rhs_val) = req.rhs().map(value_cells).unwrap_or_default();
    let hits = req
        .hits()
        .map(|h| h.inner().to_string())
        .unwrap_or_default();
    [
        flag, lhs_type, lhs_size, lhs_val, operator, rhs_type, rhs_size, rhs_val, hits,
    ]
}

/// Renders a [`Flag`] using its variant name (e.g. `PauseIf`, `AddSource`).
fn flag_cell(flag: Flag) -> String {
    match flag {
        Flag::ConditionFlag(c) => format!("{c:?}"),
        Flag::ArithmeticFlag(a) => format!("{a:?}"),
    }
}

/// Renders an [`Operator`] as a display string.
fn operator_cell(operator: Operator) -> String {
    match operator {
        Operator::Condition(c) => c.to_string(),
        Operator::Arithmetic(a) => a.to_string(),
    }
}

/// Splits a [`TypedValue`] into its `(type, size, val)` triple.
fn value_cells(value: TypedValue) -> (String, String, String) {
    match value {
        TypedValue::Memory(memory) => (
            format!("{:?}", memory.access_mode()),
            format!("{:?}", memory.size()),
            format!("0x{:x}", memory.address()),
        ),
        TypedValue::Integer(n) => ("Value".to_string(), String::new(), n.to_string()),
        TypedValue::Float(f) => ("Float".to_string(), String::new(), f.to_string()),
        TypedValue::Recall => ("Recall".to_string(), String::new(), "Recall".to_string()),
    }
}

/// Renders the core and alt groups of a [`Requirements`] as requirement tables.
pub(crate) fn render_requirements(f: &mut fmt::Formatter<'_>, group: &Requirements) -> fmt::Result {
    writeln!(
        f,
        "  Core:\n{}",
        RequirementsPreview(std::slice::from_ref(group.core()))
    )?;
    for (i, alt) in group.alt_groups().iter().enumerate() {
        writeln!(
            f,
            "  Alt {}:\n{}",
            i + 1,
            RequirementsPreview(std::slice::from_ref(alt))
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_requirement(s: &str) -> Requirement {
        s.parse().unwrap()
    }

    fn single_chain(req: Requirement) -> Chain {
        Chain::from(vec![req])
    }

    #[test]
    fn smoke_renders_condition_with_all_columns() {
        let req = parse_requirement("0xH1234=50");
        let chain = single_chain(req);
        let preview = RequirementsPreview(&[chain]).to_string();
        let lines: Vec<&str> = preview.lines().collect();
        assert_eq!(lines.len(), 5);
        assert!(lines[1].contains(HEADER_ID));
        assert!(lines[1].contains(HEADER_FLAG));
        assert!(lines[3].contains("Bits8"));
        assert!(lines[3].contains("0x1234"));
        assert!(lines[3].contains('='));
        assert!(lines[3].contains("50"));
    }

    #[test]
    fn enumerates_rows_starting_at_1() {
        let chain: Chain = vec![
            parse_requirement("0xH1234=1"),
            parse_requirement("0xH1234=2"),
            parse_requirement("0xH1234=3"),
        ]
        .into();
        let rendered = RequirementsPreview(&[chain]).to_string();
        let lines: Vec<&str> = rendered.lines().collect();
        assert!(lines[3].contains(" 1 "));
        assert!(lines[4].contains(" 2 "));
        assert!(lines[5].contains(" 3 "));
    }

    #[test]
    fn renders_condition_flag_and_hits() {
        let req = parse_requirement("P:0xH1234>=50.10.");
        let preview = RequirementsPreview(&[single_chain(req)]).to_string();
        assert!(preview.contains("PauseIf"));
        assert!(preview.contains(">="));
        assert!(preview.contains("10"));
    }

    #[test]
    fn renders_arithmetic_and_accumulator() {
        let with_op =
            RequirementsPreview(&[single_chain(parse_requirement("A:0xH1234+50"))]).to_string();
        assert!(with_op.contains("AddSource"));
        assert!(with_op.contains('+'));
        assert!(with_op.contains("50"));

        let without_op =
            RequirementsPreview(&[single_chain(parse_requirement("A:0xH1234"))]).to_string();
        assert!(!without_op.contains('+'));
    }

    #[test]
    fn columns_are_padded_to_widest_cell() {
        let rendered = Table::new(["A", "B"]).row(["x", "longer"]).to_string();
        let lines: Vec<&str> = rendered.lines().collect();
        for line in &lines {
            assert_eq!(line.chars().count(), lines[0].chars().count());
        }
    }

    #[test]
    fn empty_table_renders_headers_only() {
        let rendered = Table::new(["A", "B"]).to_string();
        let lines: Vec<&str> = rendered.lines().collect();
        assert_eq!(lines.len(), 4);
        assert!(lines[1].contains('A'));
    }

    #[test]
    fn dump_example() {
        let achievement: Chain = vec![
            parse_requirement("P:0xH0010=1.5."),
            parse_requirement("0xH0020>=50"),
            parse_requirement("A:0xH0030*2"),
        ]
        .into();
        println!(
            "\n--- multi-condition achievement ---\n{}",
            RequirementsPreview(&[achievement])
        );

        let accumulator: Chain = vec![
            parse_requirement("A:d0xH0010+1"),
            parse_requirement("A:d0xH0020+1"),
            parse_requirement("0xH0030>=100"),
        ]
        .into();
        println!(
            "\n--- accumulator chain (Delta) ---\n{}",
            RequirementsPreview(&[accumulator])
        );

        let access_modes: Chain = vec![
            parse_requirement("b0xH0010=5"),
            parse_requirement("p0xH0020=10"),
            parse_requirement("~0xH0030=15"),
            parse_requirement("d0xH0040=20"),
        ]
        .into();
        println!(
            "\n--- all access modes ---\n{}",
            RequirementsPreview(&[access_modes])
        );
    }
}

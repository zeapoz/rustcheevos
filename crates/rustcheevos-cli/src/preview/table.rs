//! Unicode box-drawing table.

use std::fmt;

/// Rounded top-left corner: `╭`.
pub(crate) const TOP_LEFT: char = '\u{256D}';
/// Rounded top-right corner: `╮`.
pub(crate) const TOP_RIGHT: char = '\u{256E}';
/// Rounded bottom-left corner: `╰`.
pub(crate) const BOTTOM_LEFT: char = '\u{2570}';
/// Rounded bottom-right corner: `╯`.
pub(crate) const BOTTOM_RIGHT: char = '\u{256F}';
/// Horizontal box-drawing line: `─`.
pub(crate) const HORIZONTAL: char = '\u{2500}';
/// Vertical box-drawing line: `│`.
pub(crate) const VERTICAL: char = '\u{2502}';
/// T-junction pointing down: `┬`.
pub(crate) const T_DOWN: char = '\u{252C}';
/// T-junction pointing up: `┴`.
pub(crate) const T_UP: char = '\u{2534}';
/// T-junction pointing right: `├`.
pub(crate) const T_RIGHT: char = '\u{251C}';
/// T-junction pointing left: `┤`.
pub(crate) const T_LEFT: char = '\u{2524}';
/// Cross junction: `┼`.
pub(crate) const CROSS: char = '\u{253C}';

/// A simple Unicode box-drawing table.
#[derive(Debug)]
pub struct Table {
    /// The column headers.
    headers: Vec<String>,
    /// The body rows.
    rows: Vec<Vec<String>>,
}

impl Table {
    /// Creates a new table with the given column headers.
    #[must_use]
    pub fn new(headers: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            headers: headers.into_iter().map(Into::into).collect(),
            rows: Vec::new(),
        }
    }

    /// Adds a row of cells to the table.
    #[must_use]
    pub fn row(mut self, cells: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.rows.push(cells.into_iter().map(Into::into).collect());
        self
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let widths = column_widths(&self.headers, &self.rows);
        let border = |left: char, mid: char, right: char| -> String {
            render_border(left, mid, right, &widths)
        };

        writeln!(f, "{}", border(TOP_LEFT, T_DOWN, TOP_RIGHT))?;
        writeln!(f, "{}", render_row(&self.headers, &widths))?;
        writeln!(f, "{}", border(T_RIGHT, CROSS, T_LEFT))?;

        for row in &self.rows {
            writeln!(f, "{}", render_row(row, &widths))?;
        }

        write!(f, "{}", border(BOTTOM_LEFT, T_UP, BOTTOM_RIGHT))
    }
}

/// Computes the display width of each column, using the widest cell.
fn column_widths(headers: &[String], rows: &[Vec<String>]) -> Vec<usize> {
    let mut widths: Vec<_> = headers.iter().map(String::len).collect();
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            if let Some(width) = widths.get_mut(i) {
                *width = (*width).max(cell.chars().count());
            }
        }
    }
    widths
}

/// Renders a single table row with vertical separators and padded cells.
fn render_row(cells: &[String], widths: &[usize]) -> String {
    let mut out = String::new();
    out.push(VERTICAL);
    for (cell, &width) in cells.iter().zip(widths) {
        out.push(' ');
        out.push_str(&pad(cell, width));
        out.push(' ');
        out.push(VERTICAL);
    }
    out
}

/// Renders a horizontal border line using the given corner and junction characters.
fn render_border(left: char, mid: char, right: char, widths: &[usize]) -> String {
    let mut out = String::new();
    out.push(left);
    for (i, &width) in widths.iter().enumerate() {
        if i > 0 {
            out.push(mid);
        }
        for _ in 0..(width + 2) {
            out.push(HORIZONTAL);
        }
    }
    out.push(right);
    out
}

/// Right-pads `s` with spaces to the given character width.
fn pad(s: &str, width: usize) -> String {
    let chars = s.chars().count();
    if chars >= width {
        s.to_string()
    } else {
        let mut out = String::with_capacity(s.len() + (width - chars));
        out.push_str(s);
        for _ in 0..(width - chars) {
            out.push(' ');
        }
        out
    }
}

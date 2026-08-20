//! Leaderboard preview.

use std::fmt;

use rustcheevos::types::leaderboard::Leaderboard;
use rustcheevos::types::requirements::Requirements;

use crate::preview::PreviewOptions;
use crate::preview::assets::requirements::render_requirements;
use crate::preview::format_separator;

/// A preview of a leaderboard.
#[derive(Debug, Clone)]
pub struct LeaderboardPreview<'a> {
    /// The leaderboard to preview.
    pub leaderboard: &'a Leaderboard,
    /// Rendering options.
    pub options: PreviewOptions,
}

/// Writes a labeled section of requirement tables.
fn write_section(
    f: &mut fmt::Formatter<'_>,
    label: &str,
    group: &Requirements,
    options: PreviewOptions,
) -> fmt::Result {
    writeln!(f, "  {label}:")?;
    render_requirements(f, group, options)
}

impl fmt::Display for LeaderboardPreview<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lb = self.leaderboard;
        writeln!(
            f,
            "{}",
            format_separator(&format!("Leaderboard: {}", lb.title()))
        )?;
        writeln!(f, "Description: {}", lb.description())?;
        writeln!(f, "ID: {}", lb.id())?;
        writeln!(f, "Format: {}", lb.format())?;
        writeln!(f, "Lower is better: {}", lb.lower_is_better())?;
        writeln!(f)?;
        write_section(f, "Start", lb.start(), self.options)?;
        write_section(f, "Cancel", lb.cancel(), self.options)?;
        write_section(f, "Submit", lb.submit(), self.options)?;
        write_section(f, "Value", lb.value(), self.options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustcheevos::types::leaderboard::LeaderboardFormat;
    use rustcheevos::types::memory::{MemoryRef, MemorySize};
    use rustcheevos::types::requirement::Condition;

    #[test]
    fn dump_example() {
        let lb = Leaderboard::builder("Speed Run")
            .description("Complete the level as fast as possible")
            .id(600707)
            .start(Condition::eq(MemoryRef::new(MemorySize::Bits8, 0x1234), 1))
            .cancel(Condition::eq(MemoryRef::new(MemorySize::Bits8, 0x1234), 0))
            .submit(Condition::eq(MemoryRef::new(MemorySize::Bits8, 0xABCD), 1))
            .format(LeaderboardFormat::Seconds)
            .lower_is_better(true)
            .build();
        println!(
            "\n--- leaderboard preview ---\n{}",
            LeaderboardPreview {
                leaderboard: &lb,
                options: PreviewOptions::default(),
            }
        );
    }
}

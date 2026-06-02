//! Leaderboard preview.

use std::fmt;

use rustcheevos::types::chain::ChainGroup;
use rustcheevos::types::leaderboard::Leaderboard;

use crate::preview::assets::requirements::render_chain_group;
use crate::preview::format_separator;

/// A preview of a leaderboard.
#[derive(Debug, Clone)]
pub struct LeaderboardPreview<'a>(pub &'a Leaderboard);

/// Writes a labeled section of requirement tables for a chain group.
fn write_section(f: &mut fmt::Formatter<'_>, label: &str, group: &ChainGroup) -> fmt::Result {
    writeln!(f, "{label}:")?;
    render_chain_group(f, group)
}

impl fmt::Display for LeaderboardPreview<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lb = self.0;
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
        write_section(f, "Start", lb.start())?;
        write_section(f, "Cancel", lb.cancel())?;
        write_section(f, "Submit", lb.submit())?;
        write_section(f, "Value", lb.value())
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
        println!("\n--- leaderboard preview ---\n{}", LeaderboardPreview(&lb));
    }
}

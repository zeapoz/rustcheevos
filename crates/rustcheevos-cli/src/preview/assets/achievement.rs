//! Achievement preview.

use std::fmt;

use rustcheevos::types::achievement::Achievement;

use crate::preview::assets::requirements::render_requirements;
use crate::preview::format_separator;

/// A preview of an achievement.
#[derive(Debug, Clone)]
pub struct AchievementPreview<'a>(pub &'a Achievement);

impl fmt::Display for AchievementPreview<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ach = self.0;
        writeln!(
            f,
            "{}",
            format_separator(&format!("Achievement: {}", ach.title()))
        )?;
        writeln!(f, "Description: {}", ach.description())?;
        writeln!(f, "ID: {}", ach.id())?;
        writeln!(f, "Points: {}", ach.points())?;
        writeln!(f, "Badge: {}", ach.badge_id())?;
        if let Some(tag) = ach.tag() {
            writeln!(f, "Tag: {tag:?}")?;
        }
        writeln!(f)?;
        render_requirements(f, ach.requirements())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustcheevos::types::achievement::Tag;
    use rustcheevos::types::chain::Requirements;
    use rustcheevos::types::memory::{MemoryRef, MemorySize};
    use rustcheevos::types::requirement::Condition;

    #[test]
    fn dump_example() {
        let core = Condition::eq(MemoryRef::new(MemorySize::Bits8, 0x1234), 50);
        let alt_a = [
            Condition::eq(MemoryRef::new(MemorySize::Bits8, 0x10), 1),
            Condition::eq(MemoryRef::new(MemorySize::Bits8, 0x20), 2),
        ];
        let alt_b = [Condition::eq(MemoryRef::new(MemorySize::Bits16, 0x30), 3)];

        let mut requirements = Requirements::new(core);
        requirements.push_alt_group(alt_a);
        requirements.push_alt_group(alt_b);

        let achievement = Achievement::builder("Alpha Amateur")
            .description("Earn a Bronze medal or higher on every planet of the Alpha galaxy")
            .requirements(requirements)
            .badge_id(12345)
            .points(3)
            .id(600707)
            .tag(Tag::Progression)
            .build();
        println!(
            "\n--- achievement preview ---\n{}",
            AchievementPreview(&achievement)
        );
    }
}

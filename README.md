# rustcheevos

A library for building achievement sets and more for [RetroAchievements](https://retroachievements.org/)
using [Rust](https://www.rust-lang.org/).

The core idea is to allow building achievements using small composable chains of conditions to reuse logic
while remaining in full control of the output using a fluent chain-like API.

## Getting Started

To use rustcheevos, add the following to your `Cargo.toml` file:

```toml
[dependencies]
rustcheevos = { git = "https://github.com/zeapoz/rustcheevos.git" }
```

## Usage

### Example Program

```rust
use rustcheevos::prelude::*;
use rustcheevos::{chain, add_address, delta, measured, bits8};

// It's recommended to define the game ID and name as constants at the top of the file.
const GAME_ID: &str = "20374";
const GAME_NAME: &str = "Geometry Wars: Galaxies";

// Logic chains can be defined as small composable and reusable functions.
fn in_game() -> Chain {
    chain!(bits8!(0x1234).eq(1))
}

// Complex logic chains can be made simpler by returning pending chains that allow
// for modiyfing the last condition in the chain.
fn current_level() -> PendingChain<MemoryRef> {
    chain!(
        add_address!(bits8!(0x16).mul(2)),
        bits8!(0x2345),
    )
}

// Use flags like delta! fluidly like you would in the achievement editor.
fn just_beat_level(level_id: u32) -> Chain {
    chain!(
        delta!(current_level()).eq(level_id),
        current_level().eq(level_id + 1),
    )
}

fn main() {
    let mut game = Game::new(GAME_ID, GAME_NAME);

    // Define an achievement by combining conditions.
    let achievement = Achievement::new(
        "First Step",
        "Complete the tutorial level",
        chain!(
            just_beat_level(1),
            in_game(),
        ),
        5,
    );
    game.add(achievement);

    // Create a simple rich presence.
    let mut rich_presence = RichPresence::new();

    // Register lookup tables.
    let mut table = LookupTable::new("Level");
    table.add_entry(Entry::new(1, "Level 1"));
    table.add_entry(Entry::new(2..=3, "Level 2"));
    table.set_fallback("Main Menu");

    // This returns a macro call handle than can be used directly in format! strings.
    let stage = rich_presence.register_lookup(table, bits8!(0x1234));
    rich_presence.add_static_display("Currently in {stage}");

    game.set_rich_presence(rich_presence);

    // Export to a directory.
    let directory = std::env::temp_dir().join("rustcheevos_example");
    std::fs::create_dir_all(&directory).unwrap();
    game.export(&directory).unwrap();
}
```



## Documentation

To build the documentation and view locally, run:

```sh
cargo doc --open
```

## Contributing

Contributions are welcome! Whether it's bug reports, feature requests, or code, feel free to open an issue on the [GitHub repository](https://github.com/zeapoz/rustcheevos).

When reporting a bug, please include:
- A clear description of the issue
- Steps to reproduce it
- Your Rust version and toolchain

When requesting a feature, please describe:
- The use case or problem you're trying to solve
- How you envision it working

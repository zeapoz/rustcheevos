# Rustcheevos CLI

This crate provides command-line tools to help with the development of
Rustcheevos sets.

## Getting Started

To install the CLI, run:

```sh
cargo install --git https://github.com/zeapoz/rustcheevos.git --bin rustcheevos-cli
```

## Usage

### Importing Code Notes

The `import` subcommand can be used to import code notes from a JSON file and generate a Rust module.

For example, to import code notes from `20374-Notes.json` and output to `memory.rs`, run:

```sh
rustcheevos-cli import -i path/to/20374-Notes.json -o memory.rs
```

The `import` subcommand supports the following options:

- `-i, --input <INPUT>`: Path to the input JSON file containing code notes.
- `-o, --output <OUTPUT>`: Path to the output Rust file to generate.
- `--no-docs`: Omit doc comments from generated functions.
- `--address <ADDRESS>`: Filter to a single note by address (hex, e.g. 0x1234).
- `--range <RANGE>`: Filter to a range of addresses (e.g. 0x1000..0x2000 or 0x1000..=0x2000).
- `--format <FORMAT>`: Output format for generated code. Options are `function` (default, generates `pub const fn` functions) or `const` (generates `pub const` constants).

## Embeddable CLI for Rustcheevos Projects

This crate can also be used as a library to add convenient CLI functions to Rustcheevos projects. This allows you to export your game assets with commands like `cargo run -- export`.

### Setup

Add the dependency to your Rustcheevos project's `Cargo.toml`:

```toml
[dependencies]
rustcheevos = { git = "https://github.com/zeapoz/rustcheevos" }
rustcheevos-cli = { git = "https://github.com/zeapoz/rustcheevos" }
```

### Usage

In your `src/main.rs`, replace your manual export logic with `RustcheevosCli`:

```rust
use rustcheevos::types::game::GameData;
use rustcheevos_cli::{RustcheevosCli, CliError};

fn main() -> Result<(), CliError> {
    let mut game_data = GameData::new(1234, "My Game");

    // ... add your achievements, leaderboards, rich presence ...

    RustcheevosCli::parse().run(&game_data)
}
```

### Available Commands

| Command | Description |
|---------|-------------|
| `export` | Export game assets to disk |
| `readme` | Generate a README.md for the game |

#### Export

```sh
cargo run -- export                  # exports to ./output
cargo run -- export -o /tmp/assets   # exports to specified directory
cargo run -- export -a "zeapoz"      # exports with custom author field
```

The `export` subcommand supports the following options:

- `-o, --output <DIR>`: Output directory for exported files (default: `output`).
- `-a, --author <AUTHOR>`: Achievement author for exported files (default: `Rustcheevos`).

#### Readme

```sh
cargo run -- readme                          # generates README.md in current directory
cargo run -- readme -o docs/README.md        # generates README at specified path
cargo run -- readme --hashes hashes.txt      # generates README with supported hashes section
```

The `readme` subcommand supports the following options:

- `-o, --output <PATH>`: Output path for the generated README (default: `README.md`).
- `--hashes <PATH>`: Path to a file containing supported hashes (format: `hash, name` per line).

The generated README includes:
- Game title and RetroAchievements link
- Table of contents (dynamic based on available content)
- Supported hashes table (if `--hashes` is provided)
- Achievements table with badge, name, description, points, and tag
- Leaderboards table with name and format
- Rich presence display strings

# License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.

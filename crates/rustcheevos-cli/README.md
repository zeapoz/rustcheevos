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

# License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.

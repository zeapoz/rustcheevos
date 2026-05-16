//! Rustcheevos CLI.
//!
//! This crate provides command-line tools to help with the development of
//! Rustcheevos sets.

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use eyre::Result;

use crate::import::{NoteFilter, OutputFormat, import};

mod import;

/// Available subcommands for the CLI.
#[derive(Debug, Subcommand)]
enum Command {
    /// Import code notes from a JSON file and generate Rust memory references.
    #[command(arg_required_else_help(true))]
    Import {
        /// Path to the input JSON file containing code notes.
        #[arg(long, short)]
        input: PathBuf,
        /// Path to the output Rust file to generate.
        #[arg(long, short)]
        output: PathBuf,
        /// Omit doc comments from generated functions.
        #[arg(long)]
        no_docs: bool,
        /// Filter to a single note by address (hex, e.g. 0x1234).
        #[arg(long, group = "filter")]
        address: Option<String>,
        /// Filter to a range of addresses (e.g. 0x1000..0x2000 or 0x1000..=0x2000).
        #[arg(long, group = "filter")]
        range: Option<String>,
        /// Output format for generated code.
        #[arg(long, value_enum, default_value_t)]
        format: OutputFormat,
    },
}

/// Command-line interface for rustcheevos-cli.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The subcommand to execute.
    #[command(subcommand)]
    command: Command,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    match &cli.command {
        Command::Import {
            input,
            output,
            no_docs,
            address,
            range,
            format,
        } => {
            let filter = if let Some(addr) = address {
                Some(NoteFilter::address(addr.as_str())?)
            } else if let Some(range) = range {
                Some(NoteFilter::range(range.as_str())?)
            } else {
                None
            };

            import(input, output, !no_docs, filter, format.clone())?;
        }
    }

    Ok(())
}

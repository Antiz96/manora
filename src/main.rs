//! Manora - A simple CLI / TUI tool to display (or save) man pages as PDFs.

// Import external modules
use clap::{ArgAction, Parser};
use std::{env, process};

// Import internal modules
mod help;
mod tmpdir;

// Parse arguments
#[derive(Parser)]
#[command(
    disable_help_flag = true,
    disable_version_flag = true
)]
struct Args {
    #[arg(short = 'h', long = "help", action = ArgAction::SetTrue)]
    help: bool,

    #[arg(short = 'V', long = "version", action = ArgAction::SetTrue)]
    version: bool,

    option: Option<String>,
    man_page: Option<String>,
    file: Option<String>,
}

fn main() {
    // Parse arguments
    let args = Args::parse();

    // Show help message if the -h / --help arg is passed
    if args.help {
        help::show_help();
        return;
    }

    // Show name and version if the -V / --version arg is passed
    if args.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return;
    }

    // Create temporary working directory
    let workdir = tmpdir::create_tmpdir().unwrap_or_else(|error| {
        eprintln!(
            "Failed to create the temporary working directory:\n{}",
            error
        );
        process::exit(4);
    });
}

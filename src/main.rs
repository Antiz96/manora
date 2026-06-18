//! Manora - A simple CLI / TUI tool to display (or save) man pages as PDFs.

// Import external modules
use log::{error, info};
use std::env;

// Import internal modules
mod help;

fn main() -> anyhow::Result<()> {
    // Initialize logger
    env_logger::init();

    // Parse arguments
    let args: Vec<String> = env::args().collect();
    let has_arg = |flag: &str| args.iter().any(|arg| arg == flag);

    // Show help message if the -h / --help arg is passed
    if has_arg("-h") || has_arg("--help") {
        help::show_help();
        return Ok(());
    }

    // Show name and version if the -V / --version arg is passed
    if has_arg("-V") || has_arg("--version") {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    Ok(())
}

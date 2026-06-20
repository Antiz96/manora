//! Manora - A simple CLI / TUI tool to display (or save) man pages as PDFs.

// Import external modules
use clap::Parser;
use std::{env, process};

// Import internal modules
mod help;
mod menu;
mod print;
mod save;
mod tmpdir;

// Argument parser
#[derive(Parser)]
#[command(
    disable_help_flag = true,
    disable_version_flag = true,
    allow_hyphen_values = true
)]
struct Args {
    // Options / flags
    #[arg(short = 'm', long)]
    menu: bool,

    #[arg(short = 's', long)]
    save: bool,

    #[arg(short = 'h', long)]
    help: bool,

    #[arg(short = 'V', long)]
    version: bool,

    // Positional arguments
    #[arg(value_name = "ARGS")]
    pos_args: Vec<String>,
}

fn main() {
    // Parse arguments
    let args = Args::parse();
    let no_args =
        args.pos_args.is_empty() && !args.menu && !args.save && !args.help && !args.version;

    // Define empty (optional) and mutable man_page variable
    // Will be set either from the menu or the first CLI argument
    let mut man_page: Option<String> = None;

    // Show TUI menu to choose man page if the -m / --menu arg (or no arg) is passed
    if args.menu || no_args {
        match menu::show_menu() {
            Ok(page) => man_page = Some(page),
            Err(error) => {
                eprintln!("{}",
                    error
                );
                process::exit(1);
            }
        }
    }

    // Save the man page as a PDF file if the -s / --save arg is passed
    if args.save {
        let man_page = args.pos_args.first().cloned().unwrap_or_else(|| {
            eprintln!("Missing man page\nTry 'manora --help' for more information");
            process::exit(3);
        });

        let file = args
            .pos_args
            .get(1)
            .cloned()
            .unwrap_or_else(|| format!("man_{}.pdf", man_page));

        save::save_man_page(&man_page, &file);
        return;
    }

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

    // Show error on invalid option
    if args
        .pos_args
        .first()
        .is_some_and(|arg| arg.starts_with('-'))
    {
        eprintln!("Invalid option\nTry 'manora --help' for more information");
        process::exit(1);
    }

    // Create temporary working directory
    let workdir = tmpdir::create_tmpdir().unwrap_or_else(|error| {
        eprintln!(
            "Failed to create the temporary working directory:\n{}",
            error
        );
        process::exit(4);
    });

    // Print man page as a PDF
    let man_page = man_page
        .or_else(|| args.pos_args.first().cloned())
        // Just making the assumption visible
        // In theory, we should never reach that expect()
        .expect("man_page should come from menu or positional argument");
    print::print2pdf(&man_page);
}

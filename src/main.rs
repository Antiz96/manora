//! Manora - A simple CLI / TUI tool to display (or save) man pages as PDFs.

// Import external modules
use clap::{ArgAction, Parser};
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
    #[arg(value_name = "ARGS")]
    args: Vec<String>,

    #[arg(short = 'm', long)]
    menu: bool,

    #[arg(short = 's', long)]
    save: bool,

    #[arg(short = 'h', long)]
    help: bool,

    #[arg(short = 'V', long)]
    version: bool,
}

// Define actions for each arguments
enum Action {
    Menu,
    Save { man_page: String, file: String },
    Help,
    Version,
    Invalid,
    Open(String),
}

fn main() {
    // Create temporary working directory
    let workdir = tmpdir::create_tmpdir().unwrap_or_else(|error| {
        eprintln!(
            "Failed to create the temporary working directory:\n{}",
            error
        );
        process::exit(4);
    });

    // Parse arguments and options
    let args = Args::parse();
    let no_args = !args.menu && !args.save && !args.help && !args.version && args.args.is_empty();

    // Assign actions and options for each arguments
    // -m / --menu or no arg
    let action = if args.menu || no_args {
        Action::Menu
    // -s / --save
    } else if args.save {
        let man_page = args.args.get(0).expect("missing man page").clone();
        let file = args
            .args
            .get(1)
            .cloned()
            .unwrap_or_else(|| format!("man_{}.pdf", man_page));
        Action::Save { man_page, file }
    // -h / --help
    } else if args.help {
        Action::Help
    // -V / --version
    } else if args.version {
        Action::Version
    } else {
        if args.args[0].starts_with('-') {
            // Invalid option
            Action::Invalid
        } else {
            // Print man page as a PDF
            Action::Open(args.args[0].clone())
        }
    };

    // Execute steps for the different actions / arguments
    match action {
        // Show TUI menu if the -m / --menu arg is passed (or if no arg is passed)
        // Then print the man page in as a pdf
        Action::Menu => {
            let man_page = menu::show_menu();
            print::print2pdf(&man_page);
        }

        // Save the man page as a PDF file if the -s / --save arg is passed
        Action::Save { man_page, file } => {
            save::save_man_page(&man_page, &file);
        }

        // Show help message if the -h / --help arg is passed
        Action::Help => {
            help::show_help();
        }

        // Show name and version if the -V / --version arg is passed
        Action::Version => {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        }

        // Show error on invalid option
        Action::Invalid => {
            eprintln!("Invalid option\nTry 'manora --help' for more information");
            process::exit(1);
        }

        // Print man as a PDF
        Action::Open(man_page) => {
            print::print2pdf(&man_page);
        }
    }
}

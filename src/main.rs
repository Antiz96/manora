//! Manora - A simple CLI / TUI tool to display (or save) man pages as PDFs.

// Import external modules
use clap::{ArgAction, Parser};
use std::{env, process};

// Import internal modules
mod help;
mod tmpdir;
mod check_man_page;
mod menu;
mod save;

// Argument parser
#[derive(Parser)]
#[command(
    disable_help_flag = true,
    disable_version_flag = true
)]
struct Args {
    #[arg(value_name = "ARGS")]
    args: Vec<String>,

    #[arg(short = 'm', long)]
    menu: bool,

    #[arg(short = 's', long)]
    save: bool,

    #[arg(short = 'h', long,)]
    help: bool,

    #[arg(short = 'V', long,)]
    version: bool,
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
    let no_args = !args.menu
        && !args.save
        && args.args.is_empty();

    // Define actions and options for each arguments
    // -m / --menu or no arg
    let action = if args.menu || no_args {
        Action::Menu
    // -s / --save
    } else if args.save {
        let man = args.args.get(0)
            .expect("missing man page")
            .clone();
        let file = args.args.get(1).cloned();
        Action::Save { man_page, file }
    // -h / --help
    } else if args.help {
        Action::Help
    // -V / --version
    } else if args.version {
        Action::Version
    } else {
    // Print man page as a PDF
        Action::Open(args.args[0].clone())
    };

    // Execute steps for the different actions / arguments
    match action {
        // Show TUI menu if the -m / --menu arg is passed (or if no arg is passed)
        // Then print the man page in as a pdf
        Action::Menu => {
            menu::show_menu();
            let man_page = menu::man_selected();
            check_man_page(&man_page);
            print2pdf(&man_page);
        }

        // Save the man page as a PDF file if the -s / --save arg is passed
        Action::Save { man_page, file } => {
            check_man_page(&man_page);
            match file {
                Some(f) => save_man_page(&man_page, &f),
                None => save_man_page(&man_page),
            }
        }

        // Show name and version if the -V / --version arg is passed
        Action::Version => {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        }

        // Show help message if the -h / --help arg is passed
        Action::Help => {
            help::show_help();
        }
}

//! Manora - A simple CLI / TUI tool to display, download and save man pages as PDF files for an easier reading

use clap::Parser;
use std::io::{self, Write};
use std::path::Path;
use std::process;

mod cachedir;
mod download;
mod help;
mod menu;
mod open;
mod save;
mod version;

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

    #[arg(short = 'd', long)]
    download: bool,

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
    let no_args = args.pos_args.is_empty()
        && !args.menu
        && !args.save
        && !args.download
        && !args.help
        && !args.version;

    // Define empty (optional) and mutable man_page variable
    // Will be set either from the menu or the first positional CLI argument
    let mut man_page: Option<String> = None;

    // Initialize menu_download_mode variable to false
    // Used later to track if the man page has been selected from the "download" mode
    // in the TUI menu (meaning it should be downloaded from https://manned.org)
    let mut menu_download_mode = false;

    // Show help message if the -h / --help arg is passed
    if args.help {
        help::show_help();
        return;
    }

    // Show name and version if the -V / --version arg is passed
    if args.version {
        version::show_version();
        return;
    }

    // Show TUI menu to choose man page if the -m / --menu arg (or no arg) is passed
    if args.menu || no_args {
        match menu::show_menu() {
            Ok((page, download_mode)) => {
                man_page = Some(page);
                menu_download_mode = download_mode;
            }
            Err(error) => {
                eprintln!("{}", error);
                process::exit(1);
            }
        }
    }

    // Download man page from https://manned.org if the -d / --download arg is passed
    // or if the man page selection was made from the "download" mode in the TUI menu
    if args.download || menu_download_mode {
        // Set man page from positional arguments
        let man_page = man_page.unwrap_or_else(|| {
            args.pos_args.first().cloned().unwrap_or_else(|| {
                eprintln!("Missing man page\nTry 'manora --help' for more information");
                process::exit(3);
            })
        });

        // Create cache directory (if it doesn't exist)
        // Needed to store the downloaded man page
        let cachedir = cachedir::create_cachedir().unwrap_or_else(|error| {
            eprintln!("Failed to create the cache directory:\n{}", error);
            process::exit(4);
        });

        // Download man page in cachedir
        download::download_man_page(&man_page, &cachedir).unwrap_or_else(|error| {
            eprintln!("Failed to download the man page:\n{}", error);
            process::exit(5);
        });

        // If used in combination with the -s / --save arg, save the downloaded man page
        if args.save {
            // Set destination file from positional arguments or fallback to default filename
            let dest_file = args
                .pos_args
                .get(1)
                .cloned()
                .unwrap_or_else(|| format!("man_{}.pdf", man_page));

            let dest_file_path = Path::new(&dest_file);

            // Ask confirmation to overwrite the destination file if it already exists
            if dest_file_path.exists() {
                print!("The {} file already exists\nOverwrite? [y/N] ", dest_file);
                io::stdout().flush().unwrap();

                let mut answer = String::new();
                io::stdin().read_line(&mut answer).unwrap();

                if !matches!(answer.trim().to_lowercase().as_str(), "y" | "yes") {
                    eprintln!("\nAborted");
                    process::exit(3);
                } else {
                    println!();
                }
            }

            // Save the downloaded the man page to the destination file
            save::save_downloaded_man_page(&man_page, &cachedir, dest_file_path).unwrap_or_else(
                |error| {
                    eprintln!("Failed to save the man page:\n{}", error);
                    process::exit(3);
                },
            );

            println!(
                "The {} man page has been downloaded from https://manned.org and saved to the {} file",
                man_page, dest_file
            );
        } else {
            // If not used in combination with `-s / --save` arg, open the downloaded man page
            open::open_downloaded_man_page(&man_page, &cachedir).unwrap_or_else(|error| {
                eprintln!("Failed to open the man page:\n{}", error);
                process::exit(1);
            });
        }

        return;
    }

    // Save the man page as a PDF file if the -s / --save arg is passed
    if args.save {
        // Set man page from positional arguments
        let man_page = args.pos_args.first().cloned().unwrap_or_else(|| {
            eprintln!("Missing man page\nTry 'manora --help' for more information");
            process::exit(3);
        });

        // Set destination file from positional arguments or fallback to default filename
        let dest_file = args
            .pos_args
            .get(1)
            .cloned()
            .unwrap_or_else(|| format!("man_{}.pdf", man_page));

        let dest_file_path = Path::new(&dest_file);

        // Ask confirmation to overwrite the destination file if it already exists
        if dest_file_path.exists() {
            print!("The {} file already exists\nOverwrite? [y/N] ", dest_file);
            io::stdout().flush().unwrap();

            let mut answer = String::new();
            io::stdin().read_line(&mut answer).unwrap();

            if !matches!(answer.trim().to_lowercase().as_str(), "y" | "yes") {
                eprintln!("\nAborted");
                process::exit(3);
            } else {
                println!();
            }
        }

        // Save the man page to the destination file
        match save::save_man_page(&man_page, dest_file_path) {
            Ok(_) => {}

            // If the man page isn't found locally, offer to download it from https://manned.org
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                eprintln!("Failed to save the man page:\n{}", error);
                print!("Would you like to try downloading it from https://manned.org? [Y/n] ");
                io::stdout().flush().unwrap();

                let mut answer = String::new();
                std::io::stdin().read_line(&mut answer).unwrap();

                if matches!(answer.trim().to_lowercase().as_str(), "" | "y" | "yes") {
                    // Create cache directory (if it doesn't exist)
                    // Needed to temporarily store the downloaded man page before moving it to the
                    // destination file
                    let cachedir = cachedir::create_cachedir().unwrap_or_else(|error| {
                        eprintln!("\nFailed to create the cache directory:\n{}", error);
                        process::exit(4);
                    });

                    // Download man page in cachedir
                    download::download_man_page(&man_page, &cachedir).unwrap_or_else(|error| {
                        eprintln!("\nFailed to download the man page:\n{}", error);
                        process::exit(5);
                    });

                    // Save the downloaded man page to the destination file
                    save::save_downloaded_man_page(&man_page, &cachedir, dest_file_path)
                        .unwrap_or_else(|error| {
                            eprintln!("\nFailed to save the man page:\n{}", error);
                            process::exit(3);
                        });
                } else {
                    eprintln!("\nAborted");
                    process::exit(5);
                }
            }

            Err(error) => {
                eprintln!("Failed to save the man page:\n{}", error);
                process::exit(3);
            }
        }

        println!(
            "The {} man page has been saved to the {} file",
            man_page, dest_file
        );
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

    // Open man page as a PDF if it is set (either from the TUI menu or the first positional argument
    // If it isn't found, offer to download it
    let man_page = man_page
        .or_else(|| args.pos_args.first().cloned())
        // Just making the assumption visible
        // In theory, we should never reach that expect()
        .expect("The man page should be set from TUI menu or the first positional argument");

    // Create cache directory (if it doesn't exist)
    // Needed to store the local or downloaded man page before opening it
    let cachedir = cachedir::create_cachedir().unwrap_or_else(|error| {
        eprintln!("Failed to create the cache directory:\n{}", error);
        process::exit(4);
    });

    // Open the man page converted as a PDF in the PDF reader
    match open::open_man_page(&man_page, &cachedir) {
        Ok(_) => {}

        // If the man page isn't found locally, offer to download it from https://manned.org
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("Failed to open the man page:\n{}", error);
            print!("Would you like to try downloading it from https://manned.org? [Y/n] ");
            io::stdout().flush().unwrap();

            let mut answer = String::new();
            std::io::stdin().read_line(&mut answer).unwrap();

            if matches!(answer.trim().to_lowercase().as_str(), "" | "y" | "yes") {
                // Download man page in cachedir
                download::download_man_page(&man_page, &cachedir).unwrap_or_else(|error| {
                    eprintln!("\nFailed to download the man page:\n{}", error);
                    process::exit(5);
                });

                // Open the downloaded man page
                open::open_downloaded_man_page(&man_page, &cachedir).unwrap_or_else(|error| {
                    eprintln!("\nFailed to open the man page:\n{}", error);
                    process::exit(1);
                });
            } else {
                eprintln!("\nAborted");
                process::exit(5);
            }
        }

        Err(error) => {
            eprintln!("Failed to open the man page:\n{}", error);
            process::exit(1);
        }
    }
}

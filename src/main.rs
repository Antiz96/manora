//! Manora - A simple CLI & TUI tool to display, download and save man pages as PDF files

use clap::Parser;
use std::io::{self, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::process;

mod cachedir;
mod convert;
mod download;
mod help;
mod menu;
mod open;
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
    // Used later to track the selected man page
    let mut man_page: Option<String> = None;

    // Initialize menu_download_mode variable
    // Used later to track if the man page has been selected from the "download" mode
    // in the TUI menu
    let mut menu_download_mode = false;

    // Define empty (optional) and mutable dl_man_page variable
    // Will eventually be populated via the "download" feature (containing the raw downloaded man page)
    // Used later to skip local conversion and convert the downloaded content instead
    let mut dl_man_page: Option<String> = None;

    // Define empty (optional) and mutable dest_file_path variable
    // Will eventually be set from the save feature or fallback to the cachedir
    // Used later to determine where the man page should be saved on the filesystem
    let mut dest_file_path: Option<PathBuf> = None;

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
        let (selected_man_page, download_mode) = menu::show_menu().unwrap_or_else(|error| {
            eprintln!("{error}");
            process::exit(1);
        });

        // Return man page & download mode status (boolean) for later operations
        man_page = Some(selected_man_page);
        menu_download_mode = download_mode;
    }

    // Download man page from https://manned.org if the -d / --download arg is passed
    // or if the man page selection was made from the "download" mode in the TUI menu
    if args.download || menu_download_mode {
        // Set man page, inherited from the menu (download mode) or positional argument
        let selected_man_page = man_page.unwrap_or_else(|| {
            args.pos_args.first().cloned().unwrap_or_else(|| {
                eprintln!("Missing man page\nTry 'manora --help' for more information");
                process::exit(3);
            })
        });

        // Download man page from https://manned.org
        let selected_dl_man_page =
            download::download_man_page(&selected_man_page).unwrap_or_else(|error| {
                eprintln!("Failed to download the man page:\n{error}");
                process::exit(5);
            });

        // Return the selected man page & downloaded man page for later operations
        man_page = Some(selected_man_page);
        dl_man_page = Some(selected_dl_man_page);
    }

    // Save the man page as a PDF file if the -s / --save arg is passed
    if args.save {
        // Set man page, inherited from previous definition or positional argument
        let selected_man_page = man_page.unwrap_or_else(|| {
            args.pos_args.first().cloned().unwrap_or_else(|| {
                eprintln!("Missing man page\nTry 'manora --help' for more information");
                process::exit(3);
            })
        });

        // Set destination file from positional argument or fallback to default filename
        let dest_file = args
            .pos_args
            .get(1)
            .cloned()
            .unwrap_or_else(|| format!("man_{selected_man_page}.pdf"));

        // Convert from string to Path
        let file_path = Path::new(&dest_file);

        // Ask confirmation to overwrite the destination file if it already exists
        if file_path.exists() {
            print!("The {dest_file} file already exists\nOverwrite? [y/N] ");
            io::stdout().flush().expect("Can't flush stdout");

            let mut answer = String::new();
            io::stdin()
                .read_line(&mut answer)
                .expect("Can't read stdin");

            if !matches!(answer.trim().to_lowercase().as_str(), "y" | "yes") {
                process::exit(3);
            } else {
                println!();
            }
        }

        // Return the man page & destination file path for later operations
        man_page = Some(selected_man_page);
        dest_file_path = Some(file_path.to_path_buf());
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

    // Set man page, inherited from previous definition or positional argument
    let man_page = man_page
        .or_else(|| args.pos_args.first().cloned())
        // Just making the assumption visible
        // In theory, we should never reach that expect() at that point
        .expect("The man page should be set from TUI menu or the first positional argument");

    // Define fallback for the destination file to cachedir (in case we don't come from
    // the -s / --save arg and it's therefore not set yet)
    let dest_file_path = dest_file_path.unwrap_or_else(|| {
        // Create cache directory (if it doesn't exist)
        // Needed to store the man page before opening it
        let cachedir = cachedir::create_cachedir().unwrap_or_else(|error| {
            eprintln!("Failed to create the cache directory:\n{error}");
            process::exit(4);
        });

        // Set the destination path to the cachedir
        cachedir.join(format!("{man_page}.pdf"))
    });

    // If we don't come from the download menu more or the -d / --download arg, search for the man page
    // locally
    if dl_man_page.is_none() {
        // Convert local man page as a PDF file and save it to the destination file
        convert::convert_man_page(&man_page, &dest_file_path).unwrap_or_else(|error| {
            // If the man page isn't found locally, offer to download it from https://manned.org
            if error.kind() == ErrorKind::Other {
                eprintln!("Failed to convert the man page as a PDF:\n{error}");
                print!("Would you like to try downloading it from https://manned.org? [Y/n] ");
                io::stdout().flush().expect("Can't flush stdout");

                let mut answer = String::new();
                io::stdin()
                    .read_line(&mut answer)
                    .expect("Can't read stdin");

                if matches!(answer.trim().to_lowercase().as_str(), "" | "y" | "yes") {
                    println!();
                    // Download man page
                    let selected_dl_man_page = download::download_man_page(&man_page)
                        .unwrap_or_else(|error| {
                            eprintln!("Failed to download the man page:\n{error}");
                            process::exit(5);
                        });

                    // Return downloaded man page for later operations
                    dl_man_page = Some(selected_dl_man_page);
                } else {
                    process::exit(5);
                }
            // For any other kind of error, return it and exit
            } else {
                eprintln!("Failed to convert the man page as a PDF:\n{error}");
                process::exit(1);
            }
        });
    }

    // If we come from the download menu mode, the -d / --download arg or if the man page was downloaded
    // after it wasn't found locally, use the downloaded man page instead
    if let Some(dl_man_page) = dl_man_page {
        // Convert downloaded man page as a PDF file and save it to the destination file
        convert::convert_downloaded_man_page(&dl_man_page, &dest_file_path).unwrap_or_else(
            |error| {
                eprintln!("Failed to convert the man page as a PDF:\n{error}");
                process::exit(3);
            },
        );
    };

    // If we come from the save arg, print info message
    if args.save {
        println!(
            "The {man_page} man page has been saved to the {} file",
            dest_file_path.display()
        );
    // Otherwise, open the man page in PDF reader
    } else {
        open::open_pdf_man_page(&dest_file_path).unwrap_or_else(|error| {
            eprintln!("Failed to open the man page:\n{error}");
            process::exit(1);
        });
    }
}

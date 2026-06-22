//! Print help message

pub fn show_help() {
    println!("manora - A simple CLI / TUI tool to display (or save) man pages as PDF files");
    println!();
    println!(
        "Run the `manora` command to display a list of all the available man pages on the system in a TUI menu, allowing to search for the one to display as a PDF."
    );
    println!(
        "Alternatively, specify the man page to open directly as an argument (e.g. `manora ls`)."
    );
    println!();
    println!("Options:");
    println!(
        "  -m, --menu                    Display the list of all the available man pages in a TUI menu, allowing you to search for the one to display as a PDF (default operation)"
    );
    println!(
        "  -s, --save <man page> <file>  Save <man page> into the <file> PDF file. If <file> isn't specified, save it to a \"man_<man page>.pdf\" file in the current directory"
    );
    println!("  -h, --help                    Display this message");
    println!("  -V, --version                 Display version information");
    println!();
    println!("For more information, see the manora(1) man page.");
}

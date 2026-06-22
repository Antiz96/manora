//! Print help message

pub fn show_help() {
    println!(
        "Manora - A simple CLI / TUI tool to display, download and save man pages as PDF files for an easier reading."
    );
    println!();
    println!(
        "Run the `manora` command to display a list of all the available man pages on the system in a TUI menu, allowing to search for the one to display as a PDF."
    );
    println!(
        "Alternatively, specify the man page to open directly as an argument (e.g. `manora ls`)."
    );
    println!();
    println!(
        "If a man page cannot be found locally, Manora offers to try to download it from <https://manned.org>."
    );
    println!();
    println!("Options:");
    println!(
        "  -m, --menu                    Display the list of all the available man pages in a TUI menu, allowing you to search for the one to display as a PDF (default operation)"
    );
    println!("  -s, --save <man page> <file>  Save <man page> into the <file> PDF file");
    println!(
        "                                If <file> isn't specified, save it to a \"man_<man page>.pdf\" file in the current directory"
    );
    println!(
        "  -d, --download                Skip local man pages lookup and directly try to download the man page from <https://manned.org> instead"
    );
    println!(
        "                                This option can be used when specifying a man page to open as an argument (`manora --download <man page>`)"
    );
    println!(
        "                                or in combination with the `-s / --save` option (`manora --download --save <man page>`)"
    );
    println!("  -h, --help                    Display this message");
    println!("  -V, --version                 Display version information");
    println!();
    println!("For more information, see the manora(1) man page.");
}

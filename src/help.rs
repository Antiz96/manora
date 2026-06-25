//! Print help message

pub fn show_help() {
    println!(
        "Manora - A simple CLI & TUI tool to display, download and save man pages as PDF files."
    );
    println!();
    println!(
        "Run the `manora` command to open a TUI menu that allows searching through local man pages, downloading man pages from <https://manned.org>, and displaying them as PDF files."
    );
    println!();
    println!("Alternatively, specify the man page to open as an argument (e.g. `manora ls`).");
    println!(
        "If the specified man page cannot be found locally, Manora offers to download it from <https://manned.org>."
    );
    println!();
    println!("Options:");
    println!(
        "  -m, --menu                    Open a TUI menu that allows searching through local man pages, downloading man pages from <https://manned.org>, and displaying them as PDF files (default operation)"
    );
    println!("  -s, --save <man page> <file>  Save <man page> into the <file> PDF file");
    println!(
        "                                If <file> isn't specified, save it to a \"man_<man page>.pdf\" file in the current directory"
    );
    println!(
        "                                If <file> already exists, ask for a confirmation to overwrite it"
    );
    println!(
        "  -d, --download                Skip searching for the man page locally and directly download it from <https://manned.org> instead"
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

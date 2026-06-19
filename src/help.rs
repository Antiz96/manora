//! Print help message

pub fn show_help() {
    println!("manora - A simple CLI / TUI tool to display (or save) man pages as PDFs.");
    println!();
    println!("You can directly specify the man page to display as a PDF.");
    println!("For instance, to display the 'ls' man page: manora ls");
    println!();
    println!("Options:");
    println!(
        "  -m, --menu                      Display the list of all the available man pages in a TUI, allowing you to search for the one to display as a PDF (default operation)"
    );
    println!(
        "  -s, --save <man page> <file>    Save <man page> into the <file> PDF file. If <man page> isn't specified, open a TUI listing every man pages to choose from. If <file> isn't specified, save to a \"man_<man page>.pdf\" file in the current directory"
    );
    println!("  -h, --help                      Display this message");
    println!("  -V, --version                   Display version information");
    println!();
    println!("For more information, see the manora(1) man page.");
}

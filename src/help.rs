//! Print help message

pub fn show_help() {
    println!("manora - A simple CLI / TUI tool to display (or save) man pages as PDFs.");
    println!();
    println!(
        "You can directly specify the man page to display as a PDF. For instance, to display the 'ls' man page:"
    );
    println!("manora ls");
    println!();
    println!("Options:");
    println!(
        "  -m, --menu                      Show a dynamic menu (via rofi or dmenu) that lists every man pages to choose from (default operation)"
    );
    println!("  -o, --output <man page> <file>  Save <man page> into the <file> PDF file");
    println!(
        "  -O, --save <man page>           Save <man page> into a \"man_<man page>.pdf\" file in the current directory; if <man page> isn't specified, open a menu via rofi or dmenu that lists every man pages to choose from"
    );
    println!("  -h, --help                      Display this message");
    println!("  -V, --version                   Display version information");
    println!();
    println!("For more information, see the manora(1) man page.");
}

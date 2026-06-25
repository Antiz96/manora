manora(1) ["Manora 2.2.0" ["Manora Manual"]]

# NAME

*Manora* - A simple CLI & TUI tool to display, download and save man pages as PDF files.

# DESCRIPTION

Run the `manora` command to open a TUI menu that allows searching through local man pages, downloading man pages from <https://manned.org>, and displaying them as PDF files.

Alternatively, specify the man page to open as an argument (e.g. `manora ls`).
If the specified man page cannot be found locally, Manora offers to download it from <https://manned.org>.

Manora opens man pages in the default PDF reader defined in XDG MIME Applications, or fallback to Zathura if no default PDF reader is set.  

Manora can also be opened from a keybinding. For instance, one can bind the `alacritty -e manora` command to a keybind, which will open the `manora` TUI menu in `alacritty`.

See below for the CLI options.

# SYNOPSIS

*manora* [OPTION] [MAN_PAGE]

# OPTIONS

*-m, --menu*
	Open a TUI menu that allows searching through local man pages, downloading man pages from <https://manned.org>, and displaying them as PDF files (default operation).

*-s, --save* <man page> <file>
	Save <man page> into the <file> PDF file.
	If <file> isn't specified, save it to a "man_<man page>.pdf" file in the current directory.
	If <file> already exists, ask for a confirmation to overwrite it.

*-d, --download* <option> <man page>
	Skip searching for the man page locally and directly download it from <https://manned.org> instead.
	This option can be used when specifying a man page to open as an argument (`manora --download <man page>`) or in combination with the `-s / --save` option (`manora --download --save <man page>`).

*-V, --version*
	Display version information.

*-h, --help*
	Display the help message.

# EXIT STATUS

*0*
	OK.

*1*
	Invalid option or unknown / missing man page.

*2*
	No PDF reader found (no PDF reader defined in XDG Mime Application and `zathura` (fallback option) isn't installed).

*3*
	Error when exporting a man page with the `-s / --save` option.

*4*
	Error when creating the cache directory.

*5*
	Error when downloading the man page from https://manned.org.

# SEE ALSO

*man*(1),
*groff*(1),
*xdg-mime*(1),
*xdg-open*(1),

# BUGS

Please report bugs to the GitHub page: https://github.com/Antiz96/manora

# AUTHOR

Robin Candau <robincandau@protonmail.com>

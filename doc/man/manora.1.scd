manora(1) ["Manora 1.3.3" ["Manora Manual"]]

# NAME

*Manora* - A simple CLI / TUI tool to display (or save) man pages as PDFs

# DESCRIPTION

A simple CLI / TUI tool to display (or save) man pages as PDFs files for an easier reading.
Manora first looks for the default PDF reader defined in XDG MIME Applications and fallback to Zathura if no default PDF reader is set.
It also allows to navigate through all the man pages available on the system through a TUI.

# SYNOPSIS

*manora* [OPTION] [MAN_PAGE]

# OPTIONS

You can directly specify the man page to display as a PDF. For instance, to display the `ls` man page:

```
manora ls
```

*-m, --menu*
	Display the list of all the available man pages in a TUI, allowing you to search for the one to display as a PDF (default operation).

*-o, --output* <man_page> <file>
	Export <man page> into the <file> PDF file.
	For instance, `manora -o ls ~/Documents/man/ls.pdf`.

*-O, --save* <man_page>
	Export <man page> into a "man_<man page>.pdf" file in the current directory.
	For instance, `manora -O ls` will save the `ls` man page in a file called `man_ls.pdf` PDF file in your current directory.
	if <man page> isn't specified, open a TUI listing every man pages to choose from.

*-V, --version*
	Display version information.

*-h, --help*
	Display the help message.

# EXIT STATUS

*0*
	OK.

*1*
	Invalid option or unknown man page.

*2*
	No PDF reader found (no PDF reader defined in XDG Mime Application and `zathura` (fallback option) isn't installed).

*3*
	Error when exporting a man page with the `-o / --output` or `-O / --save` options.

# SEE ALSO

*man*(1),
*groff*(1),
*xdg-mime*(1),
*xdg-open*(1),
*zathura*(1)

# BUGS

Please report bugs to the GitHub page: https://github.com/Antiz96/manora

# AUTHOR

Robin Candau <robincandau@protonmail.com>

manora(1) ["Manora 2.0.0" ["Manora Manual"]]

# NAME

*Manora* - A simple CLI / TUI tool to display (or save) man pages as PDF files

# DESCRIPTION

A simple CLI / TUI tool to display (or save) man pages as PDF files for an easier reading.

Manora opens man pages in the default PDF reader defined in XDG MIME Applications, or fallback to Zathura if no default PDF reader is set.  
It also allows to navigate through all the man pages available on the system through a TUI menu.

# SYNOPSIS

*manora* [OPTION] [MAN_PAGE]

# OPTIONS

You can directly specify the man page to display as a PDF. For instance, to display the `ls` man page:

```
manora ls
```

*-m, --menu*
	Display the list of all the available man pages in a TUI, allowing you to search for the one to display as a PDF (default operation).

*-s, --save* <man_page> <file>
	Save <man page> into the <file> PDF file.
	If <file> isn't specified, save it to a "man_<man page>.pdf" file in the current directory.

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

# SEE ALSO

*man*(1),
*groff*(1),
*xdg-mime*(1),
*xdg-open*(1),

# BUGS

Please report bugs to the GitHub page: https://github.com/Antiz96/manora

# AUTHOR

Robin Candau <robincandau@protonmail.com>

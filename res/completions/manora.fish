complete -c manora -f

complete -c manora -s m -l menu -d 'Open a TUI menu that allows searching through local man pages, downloading man pages from <https://manned.org>, and displaying them as PDF files (default operation)'
complete -c manora -s s -l save -d '(Arg <man page> <file>) Save <man page> into the <file> PDF file (or a "man_<man page>.pdf" file if <file> is not specified)'
complete -c manora -s d -l download -d 'Skip searching for the man page locally and directly download it from <https://manned.org> instead (e.g. "manora --download <man page>", "manora --download --save <man page>")'
complete -c manora -s h -l help -d 'Display the help message'
complete -c manora -s V -l version -d 'Display version information'

function _manora_man_pages
	man -k . | awk '{print $1}'
end

complete -c manora -a "(_manora_man_pages)"

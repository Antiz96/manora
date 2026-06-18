complete -c manora -f

complete -c manora -s m -l menu -d 'Print a menu via rofi or dmenu that lists every man pages to choose from (default operation)'
complete -c manora -s o -l output -d '(Args <man page> <file>) Save <man page> into the <file> PDF file'
complete -c manora -s O -l save -d '(Arg <man page>) Save <man page> into the "man_<man page>.pdf" file in the current directory'
complete -c manora -s h -l help -d 'Display the help message'
complete -c manora -s V -l version -d 'Display version information'

function _manora_man_pages
	man -k . | awk '{print $1}'
end

complete -c manora -a "(_manora_man_pages)"

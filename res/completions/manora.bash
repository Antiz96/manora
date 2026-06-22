_manora() {
	local arg="${2}"
	local -a opts 

	opts=('-m --menu
	       -s --save
	       -d --download
	       -h --help
	       -V --version')

	local man_pages=$(man -k . | awk '{print $1}')
	opts+=("${man_pages}")

	COMPREPLY=( $(compgen -W "${opts[*]}" -- "${arg}") )
}

complete -F _manora manora

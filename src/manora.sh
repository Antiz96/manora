#!/bin/bash

# manora: A simple CLI tool to display (or save) man pages as PDFs files for an easier reading.
# https://github.com/Antiz96/manora
# Licensed under the GPL-3.0 license (or any later version of that license).

# General variables
name="manora"
version="1.3.3"
option="${1}"
man_selected="${2}"
file="${3}"

# Create tmp dir if it doesn't exists
tmpdir="${TMPDIR:-/tmp}/${name}-${UID}"
mkdir -p "${tmpdir}"

# Display debug traces if the -D / --debug argument is passed
for arg in "${@}"; do
	case "${arg}" in
		-D|--debug)
			set -x
		;;
	esac
done

# Reset the option var if it is equal to -D / --debug (to avoid false negative "invalid option" error)
# shellcheck disable=SC2154
case "${option}" in
	-D|--debug)
		unset option
	;;
esac

# Definition of the help function: Print the help message
help() {
	cat <<EOF
A simple CLI tool to display (or save) man pages as PDFs files for an easier reading.

You can directly specify the man page to display as a PDF. For instance, to display the 'ls' man page:
${name} ls

Options:
  -m, --menu                      Show a dynamic menu (via rofi or dmenu) that lists every man pages to choose from (default operation)
  -o, --output <man page> <file>  Save <man page> into the <file> PDF file
  -O, --save <man page>           Save <man page> into a "man_<man page>.pdf" file in the current directory; if <man page> isn't specified, open a menu via rofi or dmenu that lists every man pages to choose from
  -D, --debug                     Display debug traces
  -h, --help                      Display this message
  -V, --version                   Display version information

For more information, see the ${name}(1) man page.
EOF
}

# Definition of the version function: Print the current version
version() {
	echo "${name} ${version}"
}

# Definition of the invalid_option function: Print an error message, ask the user to check the help and exit
invalid_option() {
	echo -e >&2 "${name}: invalid option -- '${option}'\nTry '${name} --help' for more information"
	exit 1
}

# Definition of the check_man_page function: Check if "${man_selected}" matches a man page (used in the "print_to_pdf" and "save_to_file" functions)
check_man_page() {
	if ! man -k . | awk '{print $1}' | grep -iq ^"${man_selected}"$; then
		echo -e >&2 "${name}: man page not found -- '${man_selected}'\nTry '${name} --help' for more information"
		exit 1
	fi
}	

# Definition of the print_to_pdf function: Display "${man_selected}" in a PDF file
print_to_pdf() {
	check_man_page

	# Check if a default PDF reader is set, otherwise fallback to zathura
	if [ -n "$(xdg-mime query default application/pdf)" ]; then
		pdf_reader="xdg-open"
	elif command -v zathura > /dev/null; then
		pdf_reader="zathura"
	else
		echo -e >&2 "There is no default PDF reader defined and Zathura isn't installed\nPlease, define a default PDF reader in XDG Mime Applications or install Zathura"
		exit 2
	fi

	man -Tpdf "${man_selected}" > "${tmpdir}/${man_selected}.pdf"
	"${pdf_reader}" "${tmpdir}/${man_selected}.pdf" &
}

# Definition of the menu function: Print the list of available man pages through a `rofi` or `dmenu` menu
# --- Drop that in favor of ratatui ---
menu() {
	if command -v rofi > /dev/null; then
		man_selected=$(man -k . | awk '{print $1}' | rofi -dmenu -sort)
	elif command -v dmenu > /dev/null; then
		man_selected=$(man -k . | awk '{print $1}' | dmenu -l 15)
	else
		echo -e >&2 "A dynamic menu is required to print the list of available man pages\nPlease, install rofi or dmenu"
		exit 2
	fi
}

# Definition of the save_to_file function: Save output to "${file}" (used in the "output" and "save" functions) 
save_to_file() {
	check_man_page

	if [ -f "${file}" ]; then
		echo "The ${file} file already exists"
		read -rp $'Do you want to overwrite its content? [y/N] ' answer

		case "${answer}" in
			[Yy])
				echo
			;;
			*)
				echo -e >&2 "\nAborting"
				exit 3
			;;
		esac
	fi

	man -Tpdf "${man_selected}" > "${file}"
	echo "The ${man_selected} man page has been saved to the ${file} file"
}

# Definition of the output function: Save the man page to the specified PDF file
output() {
	if [ -z "${man_selected}" ] || [ -z "${file}" ]; then
		echo -e >&2 "Please, specify a man page to export and a file to save it to: ${name} -o man_page /path/to/file\nTry '${name} --help' for more information"
		exit 3
	fi

	save_to_file
}

# Definition of the save function: Save the man page to a PDF file named "man_<command>.pdf" in the current directory
save() {
	if [ -z "${man_selected}" ]; then
		menu
	fi
	
	file="man_${man_selected}.pdf"

	save_to_file
}


# Execute the different functions depending on the option
case "${option}" in
	-m|--menu|"")
		menu && print_to_pdf
	;;
	-o|--output)
		output
	;;
	-O|--save)
		save
	;;
	-h|--help)
		help
	;;
	-V|--version)
		version
	;;
	*)
		if [ "${option:0:1}" == "-" ]; then
			invalid_option
		else
			man_selected="${option}"
			print_to_pdf
		fi
	;;
esac

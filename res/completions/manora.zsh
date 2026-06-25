#compdef manora

local -a opts
opts=(
    {-m,--menu}'[Open a TUI menu that allows searching through local man pages, downloading man pages from <https://manned.org>, and displaying them as PDF files (default operation)]'
    {-s,--save}'[(Arg <man page> <file>) Save <man page> into the <file> PDF file (or a "man_<man page>.pdf" file if <file> is not specified)]'
    {-d,--download}'[Skip searching for the man page locally and directly download it from <https://manned.org> instead (e.g. "manora --download <man page>", "manora --download --save <man page>")]'
    {-h,--help}'[Display the help message]'
    {-V,--version}'[Display version information]'
)

local man_pages=(${(f)"$(man -k . | awk '{print $1}')"})

_arguments $opts '*::man-pages:(${man_pages})'

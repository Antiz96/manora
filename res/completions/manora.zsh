#compdef manora

local -a opts
opts=(
    {-m,--menu}'[Print a menu via rofi or dmenu that lists every man pages to choose from (default operation)]'
    {-s,--save}'[(Arg <man page> <file>) Save <man page> into the <file> PDF file (or a "man_<man page>.pdf" file if <file> is not specified)]'
    {-h,--help}'[Display the help message]'
    {-V,--version}'[Display version information]'
)

local man_pages=(${(f)"$(man -k . | awk '{print $1}')"})

_arguments $opts '*::man-pages:(${man_pages})'

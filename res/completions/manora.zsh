#compdef manora

local -a opts
opts=(
    {-m,--menu}'[Print a menu via rofi or dmenu that lists every man pages to choose from (default operation)]'
    {-o,--output}'[(Args <man page> <file>) Save <man page> into the <file> PDF file]'
    {-O,--save}'[(Arg <man page>) Save <man page> into the "man_<man page>.pdf" file in the current directory]'
    {-h,--help}'[Display the help message]'
    {-V,--version}'[Display version information]'
)

local man_pages=(${(f)"$(man -k . | awk '{print $1}')"})

_arguments $opts '*::man-pages:(${man_pages})'

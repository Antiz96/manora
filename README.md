# Manora

## Table of contents

- [Description](#description)
- [Installation](#installation)
- [Usage](#usage)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [License](#license)

## Description

A simple CLI / TUI tool to display, download and save man pages as PDF files for an easier reading.

Run the `manora` command to display a list of all the available man pages on the system in a TUI menu (made with [Ratatui](https://ratatui.rs/), allowing to search for the one to display as a PDF.  
Alternatively, specify the man page to open directly as an argument (e.g. `manora ls`).

Manora opens man pages in the default PDF reader defined in [XDG MIME Applications](https://wiki.archlinux.org/title/XDG_MIME_Applications), or fallback to [Zathura](https://pwmt.org/projects/zathura/) if no default PDF reader is set.

Manora can also save / export man pages to a local PDF file, it offers to download man pages that cannot be found locally (from <https://manned.org>) and can be ran from a keybinding.

See the "[Usage](#usage)" chapter and the demo video below for more details:

<https://github.com/user-attachments/assets/c173aa9f-90e1-4744-9b76-cfdf4cf05a3f>

## Installation

### Runtime dependencies

- Required:

   - `man` (`man-db` or `mandoc`)
   - `groff`
   - `xdg-utils`

- Optional:

   - `zathura` - Used as the fallback PDF reader if no default PDF reader is set.
   - `zathura-pdf-poppler` or `zathura-pdf-mupdf` - PDF support for zathura.

### Packages

[![Packaging status](https://repology.org/badge/vertical-allrepos/manora.svg)](https://repology.org/project/manora/versions)

### Pre-compiled binary

A (statically linked) pre-compiled binary for the `x86_64 (amd64)` architecture is distributed as a [release asset](https://github.com/Antiz96/manora/releases/latest) (`manora-<release_version>-x86_64`).

The pre-compiled binary can be reproduced from source (in the sense of [reproducible builds](https://reproducible-builds.org)).  
The build environment is created and fully documented via [repro-env](https://github.com/kpcyrd/repro-env), and is tracked in this repository.

To reproduce the pre-compiled binary for a given release, [install repro-env](https://github.com/kpcyrd/repro-env#download) and run the following:

```bash
git clone https://github.com/Antiz96/manora.git
cd manora
git checkout <tag> # Where <tag> is the git tag for the targeted release, e.g. "v2.0.0"
repro-env build -- cargo build --release --target x86_64-unknown-linux-musl
sha256sum target/x86_64-unknown-linux-musl/release/manora
```

Then, compare the `sha256` hash of the built binary to the one of the pre-compiled release binary (which is also recorded in the `manora-<release_version>-x86_64.sha256` file in the release assets). Both hashes should be equal, indicating that the binary has been successfully reproduced.

Each release assets are also cryptographically signed, with the detached signature for each asset distributed as `<asset_name>.asc` (see the [MAINTAINERS.md file](https://github.com/Antiz96/manora/blob/main/MAINTAINERS.md) for a list of keys expected to emit signatures).

### Build from source

```bash
git clone https://github.com/Antiz96/manora.git
cd manora
cargo build --release
```

The built binary will be located at `./target/release/manora`.

The [man page](https://github.com/Antiz96/manora/tree/main/doc/man) can be generated with `scdoc`:

```bash
scdoc < doc/man/manora.1.scd > doc/man/manora.1
```

There are also shell completions available in the [`res/completions/`](https://github.com/Antiz96/manora/tree/main/res/completions) directory.

## Usage

Run the `manora` command to display a list of all the available man pages on the system in a TUI menu, allowing to search for the one to display as a PDF.  
Alternatively, specify the man page to open directly as an argument (e.g. `manora ls`).

Manora opens man pages in the default PDF reader defined in [XDG MIME Applications](https://wiki.archlinux.org/title/XDG_MIME_Applications), or fallback to [Zathura](https://pwmt.org/projects/zathura/) if no default PDF reader is set.

To save / export a man page to a local PDF file, run `manora --save <man_page>` where `<man_page>` is the man page to save (e.g. `manora --save ls`).  
The file will be saved as `man_<man_page>.pdf` (e.g. `man_ls.pdf`) in the current directory.  

Alternatively, specify the file to save the man page to: `manora --save <man_page> <file>` (e.g. `manora --save ls ~/Documents/man_pages/ls.pdf`).

If a man page cannot be found locally, Manora offers to (try to) download it from <https://manned.org> (whether it is to open or save it).

Manora can also be ran from a keybinding. For instance, one can bind the `alacritty -e manora` command to a keybinding, which will open the `manora` TUI menu in `alacritty` (allowing to select the man page to open in the PDF reader).

See `manora --help`, the [manora(1) man page](https://raw.githubusercontent.com/Antiz96/manora/refs/heads/main/doc/man/manora.1.scd) and the [demo video](#description) for more details.

## Documentation

See `manora --help` and the [manora(1) man page](https://raw.githubusercontent.com/Antiz96/manora/refs/heads/main/doc/man/manora.1.scd).

## Contributing

See the [contributing guidelines](https://github.com/Antiz96/manora/blob/main/CONTRIBUTING.md).

## License

Manora is licensed under the [GPL-3.0 license](https://github.com/Antiz96/manora/blob/main/LICENSE) (or any later version of that license).

# Manora

## Table of contents

- [Description](#description)
- [Installation](#installation)
- [Usage](#usage)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [License](#license)

## Description

A simple CLI tool to display (or save) man pages as PDFs files for an easier reading.  
Manora first looks for the default PDF reader defined in [XDG MIME Applications](https://wiki.archlinux.org/title/XDG_MIME_Applications) and fallback to [Zathura](https://pwmt.org/projects/zathura/) if no default PDF reader is set.  
It also allows to navigate through all the man pages available on the system through a dynamic menu via [Rofi](https://davatorium.github.io/rofi/) or [Dmenu](https://tools.suckless.org/dmenu/).

## Installation

### AUR

Install the [manora](https://aur.archlinux.org/packages/manora) AUR package (also check the list of optional dependencies for anything you may want or need).  

### From Source

Install dependencies:

- Replace `zathura-pdf-poppler` by `zathura-pdf-mupdf` if you prefer to use the `mupdf` backend.
- Replace `rofi` by `dmenu` if you prefer to use that (provided by the `suckless-tools` package on Debian / Ubuntu).
- The second line are build dependencies (not required to run `manora` once it is installed).

#### Debian / Ubuntu

```bash
sudo apt install man-db groff xdg-utils zathura zathura-pdf-poppler rofi
sudo apt install make bats scdoc
```

#### Fedora

```bash
sudo dnf install man-db groff groff-perl "perl(Compress::Zlib)" xdg-utils zathura zathura-pdf-poppler rofi
sudo dnf install make bats scdoc
```

#### Arch Linux

```bash
sudo pacman -S man-db groff xdg-utils zathura zathura-pdf-poppler rofi
sudo pacman -S --asdeps make bats scdoc
```

#### Alpine Linux

```bash
sudo apk add man-db groff xdg-utils zathura zathura-pdf-poppler rofi
sudo apk add make bats scdoc
```

#### Gentoo

```bash
sudo emerge man-db groff xdg-utils zathura zathura-pdf-poppler rofi
sudo emerge make bats scdoc
```

Download the archive of the [latest stable release](https://github.com/Antiz96/manora/releases/latest) and extract it (alternatively, you can clone this repository with `git`).

To install `manora`, go into the extracted/cloned directory and run the following command:

```bash
sudo make
sudo make test
sudo make install
```

Once the installation is complete, you may optionally clean up the directory of files generated during installation by running the following command:

```bash
sudo make clean
```

To uninstall `manora`, go into the extracted/cloned directory and run the following command:

```bash
sudo make uninstall
```

## Usage

Run the `manora` command in your terminal to display a list of all the available man pages on your system in a dynamic menu (via [Rofi](https://davatorium.github.io/rofi/) or [Dmenu](https://tools.suckless.org/dmenu/)), allowing you to search for the one to display as a PDF.

![manora](https://user-images.githubusercontent.com/53110319/226755165-3080f232-cb9f-4d5b-aa06-b18032cd8eaa.png)

Alternatively, you can directly specify the man page to open in the command (example below with the "ls" man page):

![manora_cmd](https://user-images.githubusercontent.com/53110319/226755190-9d005cbe-b893-4b96-b6c1-db97a70f3a4b.png)

The man page is opened as a PDF file in the default PDF reader defined in [XDG Mime Applications](https://wiki.archlinux.org/title/XDG_MIME_Applications) or in [Zathura](https://pwmt.org/projects/zathura/) if no default PDF reader is set.

![manora_pdf](https://github.com/Antiz96/manora/assets/53110319/fca2f8a7-c428-47f9-a1cd-e3dd295ce3ca)

You can export a specific man page in a PDF file called "man_`command`.pdf" in your current directory with the `-O` (or `--save`) option *(if you run `manora -O` without specifying which man page to save, `manora` will offer you to select one via rofi / dmenu)*:

![manora_save](https://user-images.githubusercontent.com/53110319/226755247-637c4827-9940-43e4-88e9-4978152e4cc4.png)

Alternatively, you can specify the destination file with the `-o` (or `--output`) option:

![manora_output](https://user-images.githubusercontent.com/53110319/226755261-cb4bf006-fae3-48ea-8187-8c4e1772b7b0.png)

## Documentation

See `manora --help` and the [manora(1) man page](https://github.com/Antiz96/manora/blob/main/doc/man/manora.1.scd).

## Contributing

See the [contributing guidelines](https://github.com/Antiz96/manora/blob/main/CONTRIBUTING.md).

## License

manora is licensed under the [GPL-3.0 license](https://github.com/Antiz96/manora/blob/main/LICENSE) (or any later version of that license).

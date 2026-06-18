pkgname=manora

PREFIX ?= /usr/local

.PHONY: build test install clean uninstall

build:
	# Generate man page
	scdoc < "doc/man/${pkgname}.1.scd" > "doc/man/${pkgname}.1"

test:
	# Run some simple unit tests on basic functions
	bats test/case/basic_functions.bats

install:
	# Install the main script
	install -Dm 755 "src/${pkgname}.sh" "${DESTDIR}${PREFIX}/bin/${pkgname}"

	# Install shell completions
	install -Dm 644 "res/completions/${pkgname}.bash" "${DESTDIR}${PREFIX}/share/bash-completion/completions/${pkgname}"
	install -Dm 644 "res/completions/${pkgname}.zsh" "${DESTDIR}${PREFIX}/share/zsh/site-functions/_${pkgname}"
	install -Dm 644 "res/completions/${pkgname}.fish" "${DESTDIR}${PREFIX}/share/fish/vendor_completions.d/${pkgname}.fish"

	# Install man page
	install -Dm 644 "doc/man/${pkgname}.1" "${DESTDIR}${PREFIX}/share/man/man1/${pkgname}.1"

	# Install documentation
	install -Dm 644 README.md "${DESTDIR}${PREFIX}/share/doc/${pkgname}/README.md"

clean:
	# Delete generated man page
	rm -f "doc/man/${pkgname}.1"

uninstall:
	# Delete the main script
	rm -f "${DESTDIR}${PREFIX}/bin/${pkgname}"

	# Delete shell completions
	rm -f "${DESTDIR}${PREFIX}/share/bash-completion/completions/${pkgname}"
	rm -f "${DESTDIR}${PREFIX}/share/zsh/site-functions/_${pkgname}"
	rm -f "${DESTDIR}${PREFIX}/share/fish/vendor_completions.d/${pkgname}.fish"

	# Delete man pages
	rm -f "${DESTDIR}${PREFIX}/share/man/man1/${pkgname}.1"

	# Delete documentation
	rm -rf "${DESTDIR}${PREFIX}/share/doc/${pkgname}/"

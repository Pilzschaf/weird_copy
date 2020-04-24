.POSIX:

PREFIX = /usr/local

RUSTC = rustc
RFILES = weird_copy.rs

weird_copy: ${RFILES}
	${RUSTC} ${RUSTFLAGS} ${RFILES}

clean:
	rm -f weird_copy
nuke: clean

install: weird_copy
	mkdir -p ${DESTDIR}${PREFIX}/bin
	cp weird_copy ${DESTDIR}${PREFIX}/bin
	chmod 555 ${DESTDIR}${PREFIX}/bin/weird_copy

uninstall:
	rm ${DESTDIR}${PREFIX}/bin/weird_copy

.PHONY: clean nuke

.PHONY: clean tools clean-tools run


TCLKIT_BUILD="tools/tclkit"
TCLKIT_DIST="tools/_tclkit"
SDX="tools/sdx.kit"


dist/app.bin: dist/app.vfs
	${TCLKIT_BUILD} ${SDX} wrap $@ -runtime ${TCLKIT_DIST} -vfs $<

dist/app.vfs: dist/app.kit dist
	${TCLKIT_BUILD} ${SDX} unwrap $<
	mv app.vfs $@

dist:
	mkdir -p dist

dist/app.kit: app.tcl dist
	${TCLKIT_BUILD} ${SDX} qwrap $< app.kit
	mv app.kit $@

clean:
	rm -rf dist


SDX_URL="https://chiselapp.com/user/aspect/repository/sdx/uv/sdx-20110317.kit"
TCLKIT_URL="https://www.sowaswie.de/downloads/tcl/macos/arm64/compiled-with-8.7/tclkit-Darwin64-tk-macOS-arm64.tar.bz2"

tools:
	mkdir -p tools
	curl -L ${SDX_URL} -o ${SDX}
	curl -L ${TCLKIT_URL} -o tools/tclkit.tar.bz2
	tar -xvf tools/tclkit.tar.bz2 -C tools
	rm tools/tclkit.tar.bz2
	mv tools/tclkit-Darwin64-tk ${TCLKIT_BUILD}
	cp ${TCLKIT_BUILD} ${TCLKIT_DIST}
	chmod +x tools/tclkit
	chmod +x tools/sdx.kit

clean-tools:
	rm -rf tools

run: dist/app.bin
	./dist/app.bin
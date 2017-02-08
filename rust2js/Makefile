SRC=hello

all: clean asmjs

clean:
	rm -f ${SRC}.js

asmjs:
	rustc --target=asmjs-unknown-emscripten ${SRC}.rs


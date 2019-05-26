target := jstemplate2

build:
	cargo build --release

build-debug:
	cargo build

install:
	cp target/release/${target} ~/bin/.

install-debug:
	cp target/debug/${target} ~/bin/${target}-debug

all: build install

all-debug: build-debug install-debug

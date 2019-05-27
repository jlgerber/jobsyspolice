target := jst
envfile := ./.env

build:
	cargo build --release

build-debug:
	cargo build

install:
	cp target/release/${target} ~/bin/.

install-debug:
	cp target/debug/${target} ~/bin/${target}-debug

install-env-file:
	cp ${envfile} ~/.

all: build install install-env-file

all-debug: build-debug install-debug install-env-file

test:
	cargo test --release --lib

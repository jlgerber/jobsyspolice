target := jsp
envfile := ./.env

build:
	cargo build --release

build-debug:
	cargo build

install:
ifneq (,$(wildcard ~/bin/${target}))
	rm ~/bin/${target}
endif
	cp target/release/${target} ~/bin/.
	chmod g+w ~/bin/${target}

install-debug:
ifneq (,$(wildcard ~/bin/${target}-debug))
	rm ~/bin/${target}-debug
endif
	cp target/debug/${target} ~/bin/${target}-debug
	chmod g+w ~/bin/${target}-debug

install-env-file:
	cp ${envfile} ~/.

all: build install install-env-file

all-debug: build-debug install-debug install-env-file

test:
	cargo test --release --lib

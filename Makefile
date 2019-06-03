target := jsp
target2 := jspmk
envfile := ./.env

build:
	cargo build --release

build-debug:
	cargo build

install:
ifneq (,$(wildcard ~/bin/${target}))
	rm ~/bin/${target}
endif
ifneq (,$(wildcard ~/bin/${target2}))
	rm ~/bin/${target2}
endif
	cp target/release/${target} ~/bin/.
	cp target/release/${target2} ~/bin/.
	chmod g+w ~/bin/${target}
	chmod g+w ~/bin/${target2}
	@echo ""
	@echo "REMEMBER TO CHMOD AND CHOWN JSP TO THE SERVICE ACCOUNT"
	@echo ""

install-debug:
ifneq (,$(wildcard ~/bin/${target}-debug))
	rm ~/bin/${target}-debug
endif
	cp target/debug/${target} ~/bin/${target}-debug
	chmod g+w ~/bin/${target}-debug
	@echo ""
	@echo "REMEMBER TO CHMOD AND CHOWN JSP TO THE SERVICE ACCOUNT"
	@echo ""

install-env-file:
	cp ${envfile} ~/.

all: build install install-env-file

all-debug: build-debug install-debug install-env-file

ownership:
	sudo chown jobsys ~/bin/${target}
	sudo chown jobsys ~/bin/${target2}
	sudo chmod u+s ~/bin/${target}
	sudo chmod u+s ~/bin/${target2}

test:
	cargo test --release --lib

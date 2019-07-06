target := jsp
target2 := jspmk
target3 := jspgo.csh
envfile := ./.env
# Location of the install target (by default your home dire)
location ?=~/bin

build:
	cargo build --release

build-debug:
	cargo build

install:
ifneq (,$(wildcard $(location)/${target}))
	rm $(location)/${target}
endif
ifneq (,$(wildcard $(location)/${target2}))
	rm $(location)/${target2}
endif
ifneq (,$(wildcard $(location)/${target3}))
	rm $(location)/${target3}
endif
	cp target/release/${target} $(location)/.
	cp target/release/${target2} $(location)/.
	cp ./${target3} $(location)/.

	chmod g+w $(location)/${target}
	chmod g+w $(location)/${target2}
	chmod g+w $(location)/${target3}
	@echo ""
	@echo "REMEMBER TO CHMOD AND CHOWN JSP TO THE SERVICE ACCOUNT"
	@echo ""

install-debug:
ifneq (,$(wildcard $(location)/${target}-debug))
	rm $(location)/${target}-debug
endif
	cp target/debug/${target} $(location)/${target}-debug
	chmod g+w $(location)/${target}-debug
	@echo ""
	@echo "REMEMBER TO CHMOD AND CHOWN JSP TO THE SERVICE ACCOUNT"
	@echo ""

install-env-file:
	cp ${envfile} $(location)/.

all: build install install-env-file ownership

all-debug: build-debug install-debug install-env-file ownership

ownership:
	#sudo chown root $(location)/${target}
	sudo chown root $(location)/${target2}
	#sudo chmod u+s $(location)/${target}
	sudo chmod u+s $(location)/${target2}


test:
	cargo test --release --lib

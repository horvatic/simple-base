.PHONY: build
.NOTPARALLEL:
build: clean
	cargo build

.PHONY: test
.NOTPARALLEL:
test: clean
	cargo test

.PHONY: clean
.NOTPARALLEL:
clean:
	cargo clean
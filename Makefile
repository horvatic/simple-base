.PHONY: build
.NOTPARALLEL:
build: clean
	cargo build

.PHONY: clean
.NOTPARALLEL:
clean:
	cargo clean
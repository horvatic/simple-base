.PHONY: build
.NOTPARALLEL:
build: clean
	rustc src/main.rs

.PHONY: clean
.NOTPARALLEL:
clean:
	rm -rf main
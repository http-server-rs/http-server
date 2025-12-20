TARGET ?= $(shell rustc --version --verbose | grep host | cut -d' ' -f2)

.PHONY: default

default:
	cargo r -- start

build:
	@echo "Building for target: $(TARGET)"
	cargo b --target $(TARGET)

release:
	@echo "Building with release mode for target: $(TARGET)"
	cargo b --release --target $(TARGET)

dist:
	make -C ../file-explorer-ui dist

run: dist
	cargo r -- start

.PHONY: default

default:
	@echo No default target.

dev: ui-build plugin-build
	cargo b --all && cargo r

release: plugin-build
	make -C ./crates/file-explorer-plugin release

plugin-build: ui-release
	make -C ./crates/file-explorer-plugin release

ui-build:
	make -C ./crates/file-explorer-ui build

ui-dev:
	@echo Starting File Explorer UI…
	make -C ./crates/file-explorer-ui dev

ui-fmt:
	make -C ./crates/file-explorer-ui fmt

ui-release:
	make -C ./crates/file-explorer-ui release

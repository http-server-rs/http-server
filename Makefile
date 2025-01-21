.PHONY: default

default:
	@echo No default target.

ui-build:
	make -C ./crates/file-explorer-ui release

ui-dev:
	@echo Starting File Explorer UI…
	make -C ./crates/file-explorer-ui dev

ui-fmt:
	make -C ./crates/file-explorer-ui fmt

ui-release:
	make -C ./crates/file-explorer-ui release

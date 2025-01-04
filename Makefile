.PHONY: default

default:
	@echo No default target.

dev: ui-build plugin-build
	cargo b --all && cargo r

release:
	@echo Preparing File Explorer Plugin…
	make -C ./crates/file-explorer-ui release
	plugin-build

	@echo Preparing HTTP Server…
	make -C ./crates/file-explorer-plugin release

plugin-build:
	make -C ./crates/file-explorer-plugin release

ui-build:
	@echo Building File Explorer UI…
	make -C ./crates/file-explorer-ui build

ui-dev:
	@echo Starting File Explorer UI…
	make -C ./crates/file-explorer-ui dev

ui-fmt:
	make -C ./crates/file-explorer-ui fmt

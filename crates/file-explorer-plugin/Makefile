.PHONY: default

default:
	@echo No default target.

release:
	@echo Building file-explorer-plugin…
	cargo b --release
	cp ../../target/release/libfile_explorer_plugin.dylib ../http-server/inline/file_explorer.plugin.httprs

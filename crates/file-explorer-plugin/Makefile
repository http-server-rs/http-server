.PHONY: default

default:
	@echo No default target.

release:
	@echo Building file-explorer-plugin…
	cargo b
	cp ../../target/debug/libfile_explorer_plugin.dylib ../http-server/inline/file_explorer.plugin.httprs

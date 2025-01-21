.PHONY: default

default:
	@echo No default target.

dev:
	trunk serve --config ./Trunk.toml

fmt:
	@leptosfmt --version >/dev/null 2>&1 || (echo "Error: leptosfmt is required."; exit 1)
	leptosfmt ./src/**/*.rs

release:
	trunk build --release --locked --config ./Trunk.toml

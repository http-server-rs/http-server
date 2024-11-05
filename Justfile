# Prints this help message
default:
  just --list

# Builds for release
release:
  cargo build --release --locked

# Runs Bats E2E Tests
e2e: release
  BIN=./target/release/http-server bats tests/e2e

# Runs formatting tool against Leptos source
ui-fmt:
  leptosfmt ./crates/web/src/**/*.rs

# Runs File Explorer UI for Development
ui-dev:
  cd ./crates/file-explorer-ui && trunk serve --config ./Trunk.toml

# Builds File Explorer UI for Production
ui-build:
  cd ./crates/file-explorer-ui && trunk build --release --locked --config ./Trunk.toml

dev: ui-build
  cargo b --all && cargo r

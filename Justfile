# Prints this help message
default:
  just --list

# Builds for release
release:
  cargo build --release --locked

# Runs Bats E2E Tests
e2e: release
  BIN=./target/release/http-server bats e2e

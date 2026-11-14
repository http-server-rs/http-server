set dotenv-load
set positional-arguments

target := `rustc --version --verbose | grep host | cut -d' ' -f2`

# List Tasks
default:
    just --list

# Build (debug)
build target=target:
    @echo "Building for target: {{target}}"
    cargo b --target {{target}}

# Build (release)
release target=target:
    @echo "Building with release mode for target: {{target}}"
    cargo b --release --target {{target}}

# Run http-server (File Explorer UI is embedded, no extra build step)
run port="7878" service="file-explorer":
    cargo r -- start --port {{port}} --service {{service}}

# Type-check the workspace
check:
    cargo check --workspace

# Lint with clippy (warnings are errors)
lint:
    cargo clippy --workspace -- -D warnings

# Check formatting without changing files
fmt-check:
    cargo fmt --all -- --check

# Run tests
test:
    just --justfile ./test/Justfile --working-directory ./test run

# Remove build artifacts
clean:
    cargo clean

# Runs the Development-Kit Container
dkc:
    docker pull ghcr.io/leoborai/dkc:latest
    docker run -it --rm \
        -v $(pwd):/app \
        -w /app \
        ghcr.io/leoborai/dkc:latest

# Perform formatting and linting
fmt:
    cargo clippy --fix --workspace --allow-dirty --allow-staged && cargo fmt

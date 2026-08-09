set dotenv-load
set positional-arguments

target := `rustc --version --verbose | grep host | cut -d' ' -f2`

# List Tasks
default:
    just --list

# Build (debug)
build target=target:
    just --justfile ./src/file-explorer-ui/Justfile --working-directory ./src/file-explorer-ui dist
    just --justfile ./src/http-server/Justfile --working-directory ./src/http-server build {{target}}

# Build (release)
release target=target:
    just --justfile ./src/file-explorer-ui/Justfile --working-directory ./src/file-explorer-ui release
    just --justfile ./src/http-server/Justfile --working-directory ./src/http-server release {{target}}

# Run http-server
run:
    just --justfile ./src/http-server/Justfile --working-directory ./src/http-server run

# Build File Explorer UI (release)
ui-build:
    just --justfile ./src/file-explorer-ui/Justfile --working-directory ./src/file-explorer-ui release

# Run File Explorer UI dev server
ui-dev:
    @echo Starting File Explorer UI…
    just --justfile ./src/file-explorer-ui/Justfile --working-directory ./src/file-explorer-ui dev

# Format File Explorer UI
ui-fmt:
    just --justfile ./src/file-explorer-ui/Justfile --working-directory ./src/file-explorer-ui fmt

# Build File Explorer UI (release)
ui-release:
    just --justfile ./src/file-explorer-ui/Justfile --working-directory ./src/file-explorer-ui release

# Run tests
test:
    just --justfile ./test/Justfile --working-directory ./test run

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

set dotenv-load
set positional-arguments

target := `rustc --version --verbose | grep host | cut -d' ' -f2`

# List Tasks
default:
    just --list

# Build (debug)
build:
    make -C ./src/file-explorer-ui dist
    make -C ./src/http-server build TARGET={{target}}

# Build (release)
release:
    make -C ./src/file-explorer-ui release
    make -C ./src/http-server release TARGET={{target}}

# Run http-server
run:
    make -C ./src/http-server run

# Build File Explorer UI (release)
ui-build:
    make -C ./src/file-explorer-ui release

# Run File Explorer UI dev server
ui-dev:
    @echo Starting File Explorer UI…
    make -C ./src/file-explorer-ui dev

# Format File Explorer UI
ui-fmt:
    make -C ./src/file-explorer-ui fmt

# Build File Explorer UI (release)
ui-release:
    make -C ./src/file-explorer-ui release

# Run tests
test:
    make -C ./test run

# Runs the Development-Kit Container
dkc:
    docker pull ghcr.io/leoborai/dkc:latest
    docker run -it --rm \
        -v $(pwd):/app \
        -w /app \
        ghcr.io/leoborai/dkc:latest

# Perform formatting and linting
fmt:
    cargo clippy --workspace --all && cargo fmt

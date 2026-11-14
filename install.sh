#!/bin/sh
# Installs the latest `http-server` release for this system.
#
#   curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/http-server-rs/http-server/main/install.sh | sh
#
# Options, as environment variables:
#   HTTP_SERVER_VERSION      a release tag such as v1.0.0 (default: the latest release)
#   HTTP_SERVER_INSTALL_DIR  where to put the binary (default: $HOME/.local/bin)
#
# The download is checked against the SHA-256 published next to it. Nothing is installed if the
# check fails. Windows is not covered by this script: download the release from the releases page.

set -eu

REPO="http-server-rs/http-server"
BIN="http-server"

say() { printf 'http-server-install: %s\n' "$*" >&2; }
die() { say "error: $*"; exit 1; }
need() { command -v "$1" >/dev/null 2>&1 || die "'$1' is required but not found"; }

# Saves the URL's body to the file given as $2.
fetch() {
    if command -v curl >/dev/null 2>&1; then
        curl --proto '=https' --tlsv1.2 -fsSL -o "$2" "$1"
    elif command -v wget >/dev/null 2>&1; then
        wget --https-only -q -O "$2" "$1"
    else
        die "'curl' or 'wget' is required"
    fi
}

target() {
    os="$(uname -s)"
    arch="$(uname -m)"
    case "$arch" in
        x86_64 | amd64) arch="x86_64" ;;
        aarch64 | arm64) arch="aarch64" ;;
        *) die "unsupported CPU architecture: $arch" ;;
    esac
    case "$os" in
        Linux)
            if ldd --version 2>&1 | grep -qi musl; then
                die "the Linux build needs glibc; this system uses musl (Alpine). Build from source: cargo install $BIN"
            fi
            echo "${arch}-unknown-linux-gnu"
            ;;
        Darwin) echo "${arch}-apple-darwin" ;;
        MINGW* | MSYS* | CYGWIN*) die "Windows is not supported by this script; see https://github.com/$REPO/releases" ;;
        *) die "unsupported operating system: $os" ;;
    esac
}

latest_tag() {
    need curl
    # /releases/latest redirects to /releases/tag/<tag>; reading the redirect avoids the API
    # rate limit. It ignores pre-releases, so fall back to the newest release of any kind.
    if url="$(curl --proto '=https' --tlsv1.2 -fsSL -o /dev/null -w '%{url_effective}' "https://github.com/$REPO/releases/latest" 2>/dev/null)"; then
        echo "${url##*/}"
        return
    fi
    curl --proto '=https' --tlsv1.2 -fsSL "https://api.github.com/repos/$REPO/releases?per_page=1" \
        | sed -n 's/.*"tag_name"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' | head -n 1
}

sha256_of() {
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum "$1" | cut -d ' ' -f 1
    elif command -v shasum >/dev/null 2>&1; then
        shasum -a 256 "$1" | cut -d ' ' -f 1
    else
        die "'sha256sum' or 'shasum' is required to check the download"
    fi
}

main() {
    need uname
    need tar
    need mktemp

    triple="$(target)"
    tag="${HTTP_SERVER_VERSION:-$(latest_tag)}"
    case "$tag" in v[0-9]*) ;; *) die "unexpected release tag: '$tag'. Set HTTP_SERVER_VERSION to a tag" ;; esac
    dest="${HTTP_SERVER_INSTALL_DIR:-$HOME/.local/bin}"

    name="${BIN}-${tag}-${triple}"
    base="https://github.com/$REPO/releases/download/$tag"

    tmp="$(mktemp -d)"
    trap 'rm -rf "$tmp"' EXIT INT TERM

    say "downloading $name"
    fetch "$base/$name.tar.gz" "$tmp/$name.tar.gz" || die "cannot download $base/$name.tar.gz"
    fetch "$base/$name.tar.gz.sha256" "$tmp/$name.tar.gz.sha256" || die "cannot download the checksum"

    expected="$(cut -d ' ' -f 1 "$tmp/$name.tar.gz.sha256")"
    actual="$(sha256_of "$tmp/$name.tar.gz")"
    [ -n "$expected" ] && [ "$expected" = "$actual" ] || die "checksum mismatch: expected '$expected', got '$actual'"

    tar -xzf "$tmp/$name.tar.gz" -C "$tmp"
    [ -f "$tmp/$name/$BIN" ] || die "the archive has no '$BIN' binary"

    mkdir -p "$dest"
    # Copy beside the target, then rename: an interrupted install never leaves a partial binary.
    cp "$tmp/$name/$BIN" "$dest/.$BIN.new"
    chmod 755 "$dest/.$BIN.new"
    mv -f "$dest/.$BIN.new" "$dest/$BIN"

    say "installed $BIN $tag to $dest/$BIN"
    case ":$PATH:" in
        *":$dest:"*) ;;
        *) say "add $dest to your PATH, for example: export PATH=\"$dest:\$PATH\"" ;;
    esac
}

# One call at the end, so a download cut short cannot run half a script.
main "$@"

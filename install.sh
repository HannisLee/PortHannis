#!/usr/bin/env sh
set -eu

REPO="${PORTCLI_REPO:-HannisLee/PortCLI}"
VERSION="${PORTCLI_VERSION:-latest}"
BINARY_NAME="portcli"

info() {
  printf '%s\n' "info: $*"
}

warn() {
  printf '%s\n' "warning: $*" >&2
}

fail() {
  printf '%s\n' "error: $*" >&2
  exit 1
}

need_cmd() {
  command -v "$1" >/dev/null 2>&1 || fail "missing required command: $1"
}

need_cmd curl
need_cmd find
need_cmd install
need_cmd sed
need_cmd tar
need_cmd mktemp
need_cmd uname
need_cmd mkdir
need_cmd rm

if [ -z "${PORTCLI_INSTALL_DIR:-}" ]; then
  [ -n "${HOME:-}" ] || fail "HOME is not set. Please set PORTCLI_INSTALL_DIR manually."
  INSTALL_DIR="$HOME/.local/bin"
else
  INSTALL_DIR="$PORTCLI_INSTALL_DIR"
fi

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Linux) ;;
  *) fail "unsupported OS: $OS. This installer currently supports Linux only." ;;
esac

case "$ARCH" in
  x86_64|amd64) TARGET="x86_64-unknown-linux-musl" ;;
  *) fail "unsupported architecture: $ARCH. This installer currently supports Linux x86_64 only." ;;
esac

if [ "$VERSION" = "latest" ]; then
  VERSION="$(
    curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" |
      sed -n 's/.*"tag_name"[[:space:]]*:[[:space:]]*"v\{0,1\}\([^"]*\)".*/\1/p' |
      sed -n '1p'
  )"
  [ -n "$VERSION" ] || fail "could not determine latest version for $REPO"
fi

VERSION="${VERSION#v}"
ARCHIVE="portcli-v${VERSION}-${TARGET}.tar.gz"
DOWNLOAD_URL="${PORTCLI_DOWNLOAD_URL:-https://github.com/$REPO/releases/download/v$VERSION/$ARCHIVE}"

TMP_DIR="$(mktemp -d)"

cleanup() {
  rm -rf "$TMP_DIR"
}

trap cleanup EXIT INT TERM

info "install dir: $INSTALL_DIR"
info "downloading $DOWNLOAD_URL"

curl -fsSL "$DOWNLOAD_URL" -o "$TMP_DIR/$ARCHIVE"

info "extracting $ARCHIVE"
tar -xzf "$TMP_DIR/$ARCHIVE" -C "$TMP_DIR"

BIN_PATH="$(find "$TMP_DIR" -type f -name "$BINARY_NAME" | sed -n '1p')"
[ -n "$BIN_PATH" ] || fail "archive does not contain $BINARY_NAME"

mkdir -p "$INSTALL_DIR" || fail "cannot create $INSTALL_DIR"

[ -w "$INSTALL_DIR" ] || fail "install dir is not writable: $INSTALL_DIR"

DEST="$INSTALL_DIR/$BINARY_NAME"

install -m 0755 "$BIN_PATH" "$DEST" || fail "cannot install to $DEST"

info "installed $("$DEST" --version) to $DEST"

case ":$PATH:" in
  *":$INSTALL_DIR:"*) ;;
  *)
    warn "$INSTALL_DIR is not in your PATH"
    warn "add this line to your shell profile:"
    warn "  export PATH=\"$INSTALL_DIR:\$PATH\""
    ;;
esac
#!/usr/bin/env bash

set -euo pipefail

info() { printf '[*] %s\n' "$*"; }
warn() { printf '[!] %s\n' "$*"; }
die() { printf '[x] %s\n' "$*" >&2; exit 1; }

REPO_ROOT="$(cd -- "$(dirname -- "$0")" && pwd)"
CONFIG_HOME="${XDG_CONFIG_HOME:-$HOME/.config}"
BIN_HOME="${NIRI_UTILS_BIN_HOME:-$HOME/.local/bin}"

info "Installing niri-utils (user mode)..."

# -------------------------
# Dependencies (optional)
# -------------------------
if command -v pacman >/dev/null 2>&1; then
  if [ "${NIRI_UTILS_SKIP_DEPS:-0}" != "1" ] && command -v sudo >/dev/null 2>&1; then
    info "Installing dependencies..."
    sudo pacman -S --needed \
      gtklock \
      gtklock-userinfo-module \
      gtklock-powerbar-module \
      imagemagick \
      rust || warn "Dependency install failed (continuing...)"
  else
    info "Skipping dependency install"
  fi
fi

# -------------------------
# Install niri-lock (SAFE)
# -------------------------
LOCK_DIR="$CONFIG_HOME/niri-lock"
info "Setting up niri-lock in $LOCK_DIR"

mkdir -p "$LOCK_DIR"

# copy only if not exists (safe)
[ -f "$LOCK_DIR/config.ini" ] || cp "$REPO_ROOT/niri-lock/config.ini" "$LOCK_DIR/"
[ -f "$LOCK_DIR/style.css" ] || cp "$REPO_ROOT/niri-lock/style.css" "$LOCK_DIR/"
cp "$REPO_ROOT/niri-lock/lock.sh" "$LOCK_DIR/"

chmod +x "$LOCK_DIR/lock.sh"

# fix style path
CONFIG_INI="$LOCK_DIR/config.ini"
STYLE_PATH="$LOCK_DIR/style.css"

if grep -q '^style=' "$CONFIG_INI" 2>/dev/null; then
  sed -i "s|^style=.*$|style=$STYLE_PATH|" "$CONFIG_INI"
else
  echo "style=$STYLE_PATH" >> "$CONFIG_INI"
fi

# -------------------------
# Build niri-idle
# -------------------------
command -v cargo >/dev/null || die "cargo not found"

info "Building niri-idle..."
(cd "$REPO_ROOT/niri-idle" && cargo build --release --locked)

# -------------------------
# Install binary
# -------------------------
info "Installing binary to $BIN_HOME"
mkdir -p "$BIN_HOME"

install -m 0755 \
  "$REPO_ROOT/niri-idle/target/release/niri-idle" \
  "$BIN_HOME/niri-idle"

# -------------------------
# PATH check
# -------------------------
if ! echo "$PATH" | grep -q "$BIN_HOME"; then
  warn "$BIN_HOME is not in PATH"
  warn "Add this to your shell config:"
  echo "export PATH=\"$BIN_HOME:\$PATH\""
fi

# -------------------------
# Done
# -------------------------
info "Install complete"

echo
echo "Run: niri-idle"
echo "Lock script: ~/.config/niri-lock/lock.sh"
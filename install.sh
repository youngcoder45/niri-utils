#!/bin/sh

set -e

echo "[*] Installing Nide..."

# Dependencies
sudo pacman -S --needed --noconfirm \
  gtklock \
  gtklock-userinfo-module \
  gtklock-powerbar-module \
  imagemagick \
  rust

# Install niri-lock
mkdir -p "$HOME/.config/niri-lock"
cp -r ./niri-lock/* "$HOME/.config/niri-lock/"
chmod +x "$HOME/.config/niri-lock/lock.sh"

# Build niri-idle
cd niri-idle
cargo build --release

mkdir -p "$HOME/.local/bin"
cp target/release/niri-idle "$HOME/.local/bin/"

echo "[✓] Done"

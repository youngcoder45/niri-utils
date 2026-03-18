#!/bin/sh

IMG="$HOME/Pictures/Wallpaper/Rogue.jpg"
TMP="/tmp/niri-lock.png"

/usr/bin/magick "$IMG" \
  -resize 1920x1080^ \
  -gravity center \
  -extent 1920x1080 \
  -brightness-contrast -20x-10 \
  "$TMP"

/usr/bin/gtklock \
  --daemonize \
  --config "$HOME/.config/niri-lock/config.ini" \
  --style "$HOME/.config/niri-lock/style.css" \
  --background "$TMP"

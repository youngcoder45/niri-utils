# niri-lock

`niri-lock` is a thin wrapper around `gtklock`.

## Files

Installed to `~/.config/niri-lock/`:

- `lock.sh` — launches `gtklock`
- `config.ini` — `gtklock` configuration
- `style.css` — theme/styles

## Configure a keybinding (Niri)

Example binding:

```kdl
bind "Mod+L" {
    spawn "/bin/sh -c ~/.config/niri-lock/lock.sh"
}
```

## Customization

- Edit `~/.config/niri-lock/style.css` to change fonts, layout, and background.
- Edit `~/.config/niri-lock/config.ini` to adjust gtklock modules and config.

## Notes

- The bundled `lock.sh` kills any existing `gtklock` instance first to avoid crash loops.
- If you’re using a wallpaper URL in `style.css`, make sure the file path exists on your system.


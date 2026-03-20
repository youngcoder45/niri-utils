# niri-utils

Minimal utilities for the **Niri** Wayland compositor.

This repo intentionally stays small and avoids sway-based components.

## What’s included

### niri-lock

Lockscreen wrapper around `gtklock`.

- Configuration lives in `~/.config/niri-lock/`
- Theme via `style.css`

### niri-idle

Rust-based idle daemon.

> [!WARNING]
> `niri-idle` is currently experimental and does not implement robust Wayland idle detection yet.

## Install

The installer is **user-mode**: it installs config files into `~/.config` and the `niri-idle` binary into `~/.local/bin`.

```bash
git clone https://github.com/youngcoder45/niri-utils
cd niri-utils
chmod +x install.sh
./install.sh
```

Notes:

- On Arch, `install.sh` can optionally install dependencies via `pacman`.
- On other distros, it skips dependency installation.
- It won’t overwrite your existing `~/.config/niri-lock/config.ini` or `style.css` (safe install), but it will update `lock.sh`.

Useful env vars:

- `NIRI_UTILS_SKIP_DEPS=1` to skip dependency installation even on Arch.
- `NIRI_UTILS_BIN_HOME=/some/path` to install `niri-idle` somewhere else.

More details: see docs.

- docs/INSTALL.md

## Usage

### Lock screen

Add this to your Niri config:

```kdl
bind "Mod+L" {
    spawn "/bin/sh -c ~/.config/niri-lock/lock.sh"
}
```

Docs: docs/NIRI_LOCK.md

### Idle daemon

Run manually:

```bash
niri-idle
```

Docs: docs/NIRI_IDLE.md

## Project structure

```
niri-utils/
├── niri-lock/
│   ├── lock.sh
│   ├── config.ini
│   └── style.css
├── niri-idle/
│   ├── Cargo.toml
│   ├── Cargo.lock
│   └── src/
├── docs/
├── install.sh
└── README.md
```

## Contributing

See CONTRIBUTING.md and docs/DEVELOPMENT.md.

## License

GNU General Public License v3.0

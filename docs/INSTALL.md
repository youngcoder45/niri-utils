# Installation

## Quick install

From the repo root:

```bash
chmod +x install.sh
./install.sh
```

This performs a **user-mode install**:

- `niri-lock` config is installed into `~/.config/niri-lock/` (or `$XDG_CONFIG_HOME/niri-lock/`).
- `niri-idle` is built with Cargo and installed into `~/.local/bin/` by default.

## Distro notes

- On Arch Linux, the script can optionally install dependencies using `pacman`.
- On other distros, it skips dependency installation.

## Environment variables

- `NIRI_UTILS_SKIP_DEPS=1`
  - Skip installing dependencies even if `pacman` is available.

- `NIRI_UTILS_BIN_HOME=/some/path`
  - Install the `niri-idle` binary into a custom directory.

## Safe install behavior

`install.sh` is intentionally conservative:

- If `~/.config/niri-lock/config.ini` exists, it is not overwritten.
- If `~/.config/niri-lock/style.css` exists, it is not overwritten.
- `~/.config/niri-lock/lock.sh` is updated on each install.

The script will also ensure `config.ini` points to the installed `style.css` path.

## Uninstall

There is no automatic uninstall yet.

- Remove `niri-idle` from wherever you installed it (default `~/.local/bin/niri-idle`).
- Remove `~/.config/niri-lock/` if you want to delete the lock config.


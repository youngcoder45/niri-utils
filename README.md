# niri-utils

A collection of minimal utilities for the Niri Wayland compositor.

This repository provides lightweight tools to enhance the Niri experience without relying on sway-based components.

---

## Overview

### niri-lock

A customizable lockscreen built using `gtklock`.

* Lightweight and minimal
* No swaylock dependency
* Themeable via simple configuration

---

### niri-idle

A Rust-based idle daemon.

> [!NOTE]
> This component is currently experimental and does not implement proper Wayland idle detection yet.

---

## Installation

```bash
git clone https://github.com/YOUR_USERNAME/niri-utils
cd niri-utils
chmod +x install.sh
./install.sh
```

---

## Usage

### Lock Screen

Add this to your Niri configuration:

```kdl
bind "Mod+L" {
    spawn "/bin/sh -c ~/.config/niri-lock/lock.sh"
}
```

---

### Idle Daemon

Run manually:

```bash
~/.local/bin/niri-idle
```

> [!WARNING]
> Do not enable automatic startup yet. The current implementation may trigger locks incorrectly.

---

## Project Structure

```
niri-utils/
├── niri-lock/
│   ├── lock.sh
│   ├── config.ini
│   └── style.css
│
├── niri-idle/
│   ├── Cargo.toml
│   └── src/
│
├── install.sh
└── README.md
```

---

## Requirements

* Niri (Wayland compositor)
* gtklock
* imagemagick
* Rust (for niri-idle)

---

## Roadmap

* Proper Wayland idle detection
* Improved configuration system
* AUR packages
* Additional modules and integrations

---

## License

GNU General Public License v3.0

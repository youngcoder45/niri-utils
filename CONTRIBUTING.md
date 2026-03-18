# Contributing

Thank you for your interest in contributing to this project.

This repository contains utilities for the Niri Wayland compositor. Contributions should maintain the project's goals of minimalism, modularity, and independence from sway-based tools.

---

## Guidelines

* Keep implementations simple and minimal.
* Avoid introducing unnecessary dependencies.
* Do not add sway or sway-related components.
* Follow existing project structure and conventions.
* Ensure scripts are POSIX-compliant (`/bin/sh`).

---

## Development Setup

Clone the repository:

```bash
git clone https://github.com/YOUR_USERNAME/niri-utils
cd niri-utils
```

Install dependencies:

```bash
sudo pacman -S gtklock imagemagick rust
```

---

## Making Changes

1. Fork the repository
2. Create a new branch:

```bash
git checkout -b feature/your-feature-name
```

3. Make your changes
4. Test thoroughly
5. Commit with clear messages:

```bash
git commit -m "feat: add new feature"
```

---

## Pull Requests

* Provide a clear description of changes
* Explain why the change is needed
* Keep pull requests focused and small

---

## Reporting Issues

When reporting bugs, include:

* System details (Arch version, Niri version)
* Steps to reproduce
* Expected vs actual behavior
* Relevant logs (if available)

---

## Notes

> [!NOTE]
> The `niri-idle` component is currently experimental. Contributions toward proper Wayland idle handling are welcome.

---

## Code Style

* Use consistent formatting
* Keep code readable and concise
* Prefer clarity over cleverness

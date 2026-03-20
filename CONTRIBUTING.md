# Contributing

Thanks for helping improve `niri-utils`.

This project aims to stay **minimal**, composable, and independent from sway-based tooling.

## Ground rules

- Keep changes small and focused.
- Avoid new dependencies unless clearly justified.
- Prefer simple, readable code over cleverness.
- Don’t introduce sway / swaylock / swayidle based components.

## Repo layout

- `niri-lock/` — `gtklock` wrapper + config.
- `niri-idle/` — Rust crate (experimental).
- `install.sh` — user-mode installer (currently uses bash).

## Development setup

Clone:

```bash
git clone https://github.com/youngcoder45/niri-utils
cd niri-utils
```

### Rust (niri-idle)

From `niri-idle/`:

```bash
cargo build
cargo fmt
cargo clippy -- -D warnings
cargo test
```

### Shell scripts

- `niri-lock/lock.sh` and `install.sh` are shell scripts.
- If you use bash-isms, keep the shebang as bash and avoid breaking non-interactive execution.
- Be careful with quoting, file permissions, and paths.

## Documentation

Docs live under `docs/` and should:

- Match the actual behavior of the scripts/binaries.
- Avoid distro-specific assumptions unless called out.
- Prefer short, actionable examples.

## Pull requests

- Describe what changed and why.
- Include how you tested (commands + platform).
- Keep PRs focused (one theme per PR).

## Reporting issues

When filing a bug, include:

- Distro + versions (Niri, gtklock)
- Steps to reproduce
- Expected vs actual behavior
- Logs/terminal output

> [!NOTE]
> `niri-idle` is experimental; correctness reports are especially valuable.

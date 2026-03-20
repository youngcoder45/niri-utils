# Development

## Rust workflow (niri-idle)

From `niri-idle/`:

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
cargo build --release --locked
```

## Shell scripts

- Keep scripts safe for non-interactive execution.
- Quote variables, use `set -e` / `set -u` where appropriate, and avoid surprising writes.

## Testing installer changes

Basic syntax check:

```bash
bash -n install.sh
```

Dry-run approach (manual):

- Run in a throwaway user or VM
- Verify it doesn’t overwrite existing `~/.config/niri-lock/config.ini` / `style.css`
- Verify `niri-idle` gets installed and is runnable


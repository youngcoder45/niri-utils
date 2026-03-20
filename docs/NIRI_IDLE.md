# niri-idle

`niri-idle` is an experimental idle daemon written in Rust.

> [!WARNING]
> It does not yet implement robust Wayland idle detection. Avoid enabling automatic startup until you’re confident it behaves correctly on your system.

## Run

If installed via `install.sh` (default location):

```bash
niri-idle
```

## Build from source

```bash
cd niri-idle
cargo build --release --locked
./target/release/niri-idle
```

## Configuration

If/when configuration is supported, it will be documented here.


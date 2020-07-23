# Yew Sandbox

## Quickstart

### Using the nightly Rust toolchain

```bash
# add the nightly toolchain
rustup toolchain install nightly

# use the nightly toolchain for the yew-playground directory.
rustup override set nightly
```

### Commands

Run the following command to install the required dependencies:

```bash
cargo install just watchexec simple-http-server
```

You can run `just --list` to list all possible commands.
You can also use `just help` for a bit of help.

The most important ones are `just watch-server` and `just watch-client`.

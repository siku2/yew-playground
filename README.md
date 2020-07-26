# Yew Playground

## Quickstart

### Using the nightly Rust toolchain

This project requires the Rust nightly to compile.
Use the following commands to install the nightly toolchain and set it as the active toolchain for the yew-playground directory.

```bash
# add the nightly toolchain
rustup toolchain install nightly

# use the nightly toolchain for the current directory.
rustup override set nightly
```

### Commands

Run the following command to install the required dependencies:

```bash
cargo install just watchexec
```

You can run `just --list` to list all possible commands.
You can also use `just help` for a bit of help.

The most important ones are `just watch`.

#### Building the Docker images

just server/config


## Reverse proxy

<!-- TODO -->

Requires a reverse proxy to forward `$1.<HOST>/$2` to `<HOST>/sandbox/$1/$2`.

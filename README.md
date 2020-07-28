<div align="center">

# Yew Playground

</div>

## Getting started

### Setting up the nightly toolchain

This project requires the nightly version of Rust.
Use the following commands to install the nightly toolchain and set it as the active toolchain for the _yew-playground_ directory.

```bash
# add the nightly toolchain
rustup toolchain install nightly

# use the nightly toolchain for the current directory.
rustup override set nightly
```

> This will only activate it for the _yew-playground_ directory. All your other projects are safe :).

### Installing dependencies

This project uses the command runner [just](https://github.com/casey/just) to make performing various tasks a lot easier.

Additionally, [watchexec](https://github.com/watchexec/watchexec) is used to watch for file changes.
If you don't want to manually rebuild every time you change something you should install it too.

Run the following command to install these dependencies:

```bash
cargo install just watchexec
```

### Building the Docker images

For security reasons the compiler and all other tools run in a docker container.
The images for these containers need to be built before the server can do anything.

> Technically you don't need to build them if you're solely interested in the frontend.
> Just keep in mind that you can't use most of the frontend's features without a working server.

Use the following command to build all required images:

```bash
just docker/build
```

This will take a very long time.
Go drink a coffee (or three) while you're waiting.

For more details, please visit the [docker](docker) directory.

### Running the playground

Now we're finally ready to get to the fun part.
All you need to get things running is the following command:

```bash
just watch
```

This command builds the frontend and starts the server.
As the name implies, the command watches for changes.
If you modify either the frontend or the server it will automatically update (A manual reload in the browser is still required).

If you only want to start the server without updating on every change you can use the following command:

```bash
just run
```

---

Use `just --list` to list all available commands.
There are a lot more than the ones listed above.

## Deploying

<!-- TODO -->

A reverse proxy to forward `$1.<HOST>/$2` to `<HOST>/proxy/$1/$2` is required.

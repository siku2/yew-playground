# Docker Images

<!-- TODO explain how the docker images are designed -->

## Build all images

```sh
just build
```

This will create the following docker images:

- `yewstack/playground_base`
- `compiler-stable`
- `compiler-nightly`
- `clippy`
- `rustfmt`
- `cargo-expand`

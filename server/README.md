# Server

## Design

The backend consists of a [Rocket](https://rocket.rs/) server. It handles everything from compiling to serving the frontend.

[Docker containers](https://www.docker.com/resources/what-container) are used to execute various commands safely.

The whole design is largely similar to that of [Rust Playground](https://github.com/integer32llc/rust-playground).
One key difference is that sandboxes persist on the server so that the output can be served.

ARG channel
FROM yewstack/playground_base:$channel

# precompile crates in both debug and release mode
RUN cargo build
RUN cargo build --release

RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-pack

RUN wasm-pack build --dev
RUN wasm-pack build --release

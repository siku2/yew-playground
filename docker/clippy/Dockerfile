ARG channel
FROM yewstack/playground_base:$channel

RUN rustup component add clippy
RUN cargo clippy

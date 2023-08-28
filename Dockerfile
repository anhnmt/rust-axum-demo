FROM rust:1.72-slim-buster as builder

# create a new empty shell project
RUN USER=root cargo new --bin app
WORKDIR /app

# copy over your manifests
COPY Cargo.toml ./

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*

# copy your source tree
COPY . .

# build for release
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /app

RUN addgroup --system --gid 1001 rust
RUN adduser --system --uid 1001 rust

# copy the build artifact from the build stage
COPY --from=builder --chown=rust:rust /app/target/release/rust-axum-demo ./main

USER rust

# set the startup command to run your binary
CMD ["./main"]
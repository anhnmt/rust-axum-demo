ARG RUST_VERSION=1.71

FROM rust:${RUST_VERSION}-slim-buster AS builder

# create a new empty shell project
RUN USER=root cargo new --bin app
WORKDIR /app

# copy over your manifests
COPY Cargo.toml Cargo.lock ./

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*

# copy your source tree
COPY . .

# build for release
RUN cargo build --locked --release

FROM debian:buster-slim  AS runtime
WORKDIR /app

RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid "10001" \
  appuser

# copy the build artifact from the build stage
COPY --from=builder --chown=appuser /app/target/release/rust-axum-demo ./main

USER appuser

# set the startup command to run your binary
CMD ["./main"]
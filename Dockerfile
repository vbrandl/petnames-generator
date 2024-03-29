FROM rust:latest as builder

# Make a fake Rust app to keep a cached layer of compiled crates
RUN USER=root cargo new app
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
# Needs at least a main.rs file with a main function
RUN mkdir src && echo "fn main(){}" > src/main.rs && echo "fn main(){}" > src/build.rs
# Will build all dependent crates in release mode
RUN cargo build --release

# Copy the rest
COPY . .

RUN touch src/main.rs && touch src/build.rs

# TODO, HACK: This is a temporary workaround for
# https://github.com/vbrandl/petnames-generator/issues/21
# `flyctl deploy` deletes or ignores `fly.toml` when building the image, this
# causes the repo to be dirty at build time which breaks the version info in
# the footer
RUN git config --global --add safe.directory /usr/src/app && git checkout fly.toml

# Build (install) the actual binaries
RUN cargo build --release

# Runtime image
FROM debian:stable-slim

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/src/app/target/release/petnames-generator /app/petnames-generator

# No CMD or ENTRYPOINT, see fly.toml with `cmd` override.

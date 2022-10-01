FROM rust:latest as builder

# Make a fake Rust app to keep a cached layer of compiled crates
RUN USER=root cargo new app
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
# Needs at least a main.rs file with a main function
RUN mkdir src && echo "fn main(){}" > src/main.rs
# Will build all dependent crates in release mode
RUN cargo build --release

# Copy the rest
COPY . .

RUN touch src/main.rs

# Build (install) the actual binaries
RUN cargo build --release

# Runtime image
FROM debian:stable-slim

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/src/app/target/release/petnames-fly /app/petnames-fly

# No CMD or ENTRYPOINT, see fly.toml with `cmd` override.

FROM rust:1.89-slim-bookworm

WORKDIR /app

# Copy all build dependencies.
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY build.rs ./
COPY metadata ./metadata

# Install system dependencies.
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

# Build the Rust executable.
RUN cargo build --release

# Set the entrypoint to be the Rust executable
ENTRYPOINT ["/app/target/release/c-rust-program-pairs"]

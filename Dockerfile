FROM rust:1.89-slim-bookworm

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY src ./src
COPY build.rs ./
COPY metadata ./metadata

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

RUN cargo build --release

ENTRYPOINT ["/app/target/release/c-rust-program-pairs"]

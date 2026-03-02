FROM rust:1.89-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/groqqle-rust /usr/local/bin/groqqle-rust
RUN mkdir -p /data
ENV APP_CONFIG_PATH=/data/config.json
EXPOSE 5000
CMD ["groqqle-rust", "api", "--port", "5000", "--num-results", "10", "--summary-length", "300"]

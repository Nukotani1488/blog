# ---------- Build stage ----------
FROM rust:1.93 AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
COPY . .
RUN touch src/main.rs && cargo build --release

# ---------- Runtime stage ----------
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/blog .
COPY --from=builder /app/static ./static
EXPOSE 3000
CMD ["./blog"]
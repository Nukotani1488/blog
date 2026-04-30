# ---------- Build stage ----------
FROM rust:1.93 AS builder

WORKDIR /app

# 1. Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# 2. Copy source code
COPY . .

# 3. Build the binary
RUN cargo build --release


# ---------- Runtime stage ----------
FROM debian:bookworm-slim

WORKDIR /app

# Install required runtime libs
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary
COPY --from=builder /app/target/release/blog .

# Expose port
EXPOSE 3000

# Run app
CMD ["./blog"]
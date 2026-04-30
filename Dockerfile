# ---------- Build stage ----------
FROM rust:latest AS builder

WORKDIR /app

# 1. Cache dependencies first
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# 2. Copy actual source code
COPY . .

# 3. Build real binary
RUN cargo build --release


# ---------- Runtime stage ----------
FROM debian:bookworm-slim

WORKDIR /app

# Install required runtime libs (important for some crates)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy only the compiled binary
COPY --from=builder /app/target/release/blog .

# Expose port
EXPOSE 3000

# Run app
CMD ["./blog"]
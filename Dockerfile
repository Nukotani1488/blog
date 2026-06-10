FROM --platform=$BUILDPLATFORM rust:1.95 AS builder
ARG TARGETARCH
WORKDIR /app

RUN apt-get update && apt-get install -y \
    gcc-aarch64-linux-gnu \
    gcc-x86-64-linux-gnu \
    libc6-dev-arm64-cross \
    libc6-dev-amd64-cross \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add aarch64-unknown-linux-gnu x86_64-unknown-linux-gnu

COPY .cargo .cargo
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN case "$TARGETARCH" in \
    amd64) cargo build --release --target x86_64-unknown-linux-gnu ;; \
    arm64) cargo build --release --target aarch64-unknown-linux-gnu ;; \
    esac

COPY . .
RUN case "$TARGETARCH" in \
    amd64) touch src/main.rs && cargo build --release --target x86_64-unknown-linux-gnu && cp target/x86_64-unknown-linux-gnu/release/blog /app/blog ;; \
    arm64) touch src/main.rs && cargo build --release --target aarch64-unknown-linux-gnu && cp target/aarch64-unknown-linux-gnu/release/blog /app/blog ;; \
    esac

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/blog .
COPY --from=builder /app/static ./static
EXPOSE 3000
CMD ["./blog"]
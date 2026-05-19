# syntax=docker/dockerfile:1.7

# ---------- Builder ----------
# Pin amd64 so building on Apple Silicon still produces an EC2-compatible image.
# (Switch to linux/arm64 if you move to a Graviton instance like t4g.*)
FROM --platform=linux/amd64 rust:1-bookworm AS builder
WORKDIR /app

# Copy manifests + sources, then build a release binary.
# (Dependency-only caching can be added later via cargo-chef.)
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --locked

# ---------- Runtime ----------
FROM --platform=linux/amd64 debian:bookworm-slim AS runtime
WORKDIR /app

# Run as a non-root user
RUN groupadd --system app && useradd --system --gid app --home /app app

COPY --from=builder /app/target/release/web_server /app/web_server
COPY static ./static

USER app

ENV BIND_ADDR=0.0.0.0 \
    PORT=8080

EXPOSE 8080
CMD ["/app/web_server"]

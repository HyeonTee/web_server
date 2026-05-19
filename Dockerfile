# syntax=docker/dockerfile:1.7

# ---------- Builder ----------
FROM rust:1-bookworm AS builder
WORKDIR /app

# Copy manifests + sources, then build a release binary.
# (Dependency-only caching can be added later via cargo-chef.)
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --locked

# ---------- Runtime ----------
FROM debian:bookworm-slim AS runtime
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

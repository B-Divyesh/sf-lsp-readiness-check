FROM rust:1.98-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY server ./server
COPY examples ./examples
RUN cargo build --locked --release -p lsp-readiness-api

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --create-home --uid 10001 app \
    && mkdir -p /data \
    && chown app:app /data
COPY --from=builder /app/target/release/lsp-readiness-api /usr/local/bin/lsp-readiness-api
USER app
ENV DATABASE_PATH=/data/lsp-readiness-v2.db \
    PORT=8080 \
    RUST_LOG=info
EXPOSE 8080
VOLUME ["/data"]
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 CMD curl --fail --silent http://127.0.0.1:8080/healthz >/dev/null || exit 1
ENTRYPOINT ["/usr/local/bin/lsp-readiness-api", "serve"]

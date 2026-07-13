FROM rust:1.93-bookworm AS builder

WORKDIR /build
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY crates ./crates
RUN cargo build --locked --release --package mailforagents

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install --yes --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --gid 10001 mailforagents \
    && useradd --uid 10001 --gid mailforagents --create-home mailforagents \
    && mkdir --parents /app/data \
    && chown --recursive mailforagents:mailforagents /app

COPY --from=builder /build/target/release/mailforagents /usr/local/bin/mailforagents

USER mailforagents
WORKDIR /app
ENV MFA_APP_ADDR=0.0.0.0:3200 \
    MFA_DATABASE_PATH=/app/data/app.sqlite \
    MFA_DEPLOYMENT_MODE=self-hosted \
    RUST_LOG=mailforagents=info,tower_http=info
EXPOSE 3200
VOLUME ["/app/data"]
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl --fail --silent http://127.0.0.1:3200/readyz || exit 1

ENTRYPOINT ["/usr/local/bin/mailforagents"]

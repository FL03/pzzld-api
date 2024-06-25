ARG RUST_VERSION=1.78.0

FROM rust:${RUST_VERSION}-slim-bookworm AS builder

WORKDIR /app

COPY . .

RUN \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release && \
    cp ./target/release/pzzld /

FROM debian:bookworm-slim AS final

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "10001" \
    appuser

COPY --from=builder /pzzld /usr/local/bin

RUN chown appuser /usr/local/bin/pzzld

COPY --from=builder /app/.config /opt/pzzld/.config

RUN chown -R appuser /opt/pzzld

USER appuser

ENV RUST_LOG="pzzld=debug,info"

WORKDIR /opt/pzzld
ENTRYPOINT ["pzzld"]
EXPOSE 8080/tcp
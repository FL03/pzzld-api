FROM rust:latest as builder-base

RUN apt-get update -y && \
    apt-get upgrade -y && \
    rustup update

FROM builder-base as builder

ADD . /workspace

WORKDIR /workspace

COPY . .

RUN cargo build -r -v

FROM debian:latest as runner-base

ENV BINARY=pzzld

RUN apt-get update -y && apt-get upgrade -y

COPY --from=builder /workspace/target/release/${APP_NME} /app/server
COPY --from=builder /workspace/assets /app/assets
COPY --from=builder /workspace/Puzzled.toml /app/Puzzled.toml

EXPOSE 8080/tcp

ENTRYPOINT [ "app/server" ]

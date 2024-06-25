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

ENV BINARY=pzzld \
    PORT=8080 

RUN apt-get update -y && apt-get upgrade -y

WORKDIR /app

COPY --from=builder /workspace/target/release/pzzld /pzzld
COPY --from=builder /workspace/assets /assets
COPY --from=builder /workspace/Puzzled.toml /Puzzled.toml

EXPOSE 80
EXPOSE 8080
EXPOSE ${PORT}


ENTRYPOINT [ "pzzld" ]

FROM rust:1.85-alpine AS BUILDER

RUN mkdir /app-builder
WORKDIR /app-builder

RUN apk add \
    musl-dev \
    pkgconfig \
    openssl-dev \
    curl

ENV OPENSSL_DIR=/usr

COPY Cargo.toml ./
COPY Cargo.lock ./
COPY cli cli
COPY common common
COPY entity entity
COPY migration migration
COPY server server

RUN cargo build --release

FROM alpine:3.21

RUN mkdir /app

COPY --from=BUILDER /app-builder/target/release/appledb_server /app

ENTRYPOINT [ "/app/appledb_server", "--config", "/app/config.yaml" ]


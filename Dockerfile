ARG HTTP_PROXY=""
ARG HTTPS_PROXY=""

FROM docker.io/library/rust:1.85-alpine AS rust_builder

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

FROM docker.io/library/node:18.20-alpine3.21 AS web_builder

RUN mkdir /app

WORKDIR /app

COPY web ./

RUN yarn install

RUN yarn build

FROM docker.io/library/alpine:3.21

RUN mkdir /app
RUN mkdir /app/dist

COPY --from=web_builder /app/dist /app/dist
COPY --from=rust_builder /app-builder/target/release/appledb_server /app

ENV CONFIG_PATH="/app/config.yaml"

CMD [ "/app/appledb_server"]

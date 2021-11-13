FROM rust:alpine as base

FROM base as builder

RUN apk add --no-cache gcc git musl-dev openssl-dev postgresql-dev bash && rustup target add x86_64-unknown-linux-musl
RUN cargo install sqlx-cli --no-default-features --features postgres

# Set up our environment variables so that we cross-compile using musl-libc by
# default.
ENV X86_64_UNKNOWN_LINUX_MUSL_OPENSSL_DIR=/usr/local/musl/ \
    X86_64_UNKNOWN_LINUX_MUSL_OPENSSL_STATIC=1 \
    PQ_LIB_STATIC_X86_64_UNKNOWN_LINUX_MUSL=1 \
    PG_CONFIG_X86_64_UNKNOWN_LINUX_GNU=/usr/bin/pg_config \
    PKG_CONFIG_ALLOW_CROSS=true \
    PKG_CONFIG_ALL_STATIC=true \
    LIBZ_SYS_STATIC=1 \
    TARGET=musl \
    USER=root

WORKDIR /src

# Create blank project
RUN cargo init --bin

# Setup blank project dependencies for build caching
COPY Cargo.toml Cargo.lock /src/

RUN cargo b --target x86_64-unknown-linux-musl

COPY sqlx-data.json /src
COPY src /src/src
COPY migrations /src/migrations

RUN touch src/*.rs && cargo b --target x86_64-unknown-linux-musl

FROM base

ENV RUN_MODE="docker"

RUN apk add --no-cache postgresql-libs bash

COPY --from=builder /src/target/x86_64-unknown-linux-musl/debug/web-of-trust-backend /api/
COPY --from=builder /src/migrations /api/migrations/
COPY config/ /api/config/

WORKDIR /api

CMD ["bash", "docker.sh"]

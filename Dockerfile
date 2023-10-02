FROM rust:slim AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev protobuf-compiler
RUN update-ca-certificates

ARG CRATE_SERVER=server
ARG CRATE_CORE=api-core
ARG CRATE_INTERFACE=api-interface
ARG CRATE_DB=api-db
ARG CRATE_UTILS=api-utils

WORKDIR /usr/src/app

COPY ./Cargo.lock .
COPY ./Cargo.toml .

WORKDIR /usr/src/app/crates
RUN cargo new ${CRATE_SERVER}
RUN cargo new --lib ${CRATE_DB}
RUN cargo new --lib ${CRATE_UTILS}
RUN cargo new --lib ${CRATE_INTERFACE}
RUN cargo new --lib ${CRATE_CORE}

WORKDIR /usr/src/app

COPY ./crates/${CRATE_SERVER}/Cargo.toml crates/${CRATE_SERVER}
COPY ./crates/${CRATE_CORE}/Cargo.toml crates/${CRATE_CORE}
COPY ./crates/${CRATE_INTERFACE}/Cargo.toml crates/${CRATE_INTERFACE}
COPY ./crates/${CRATE_DB}/Cargo.toml crates/${CRATE_DB}
COPY ./crates/${CRATE_UTILS}/Cargo.toml crates/${CRATE_UTILS}

RUN cargo fetch

COPY . .

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/server-ads ./
CMD [ "./server-ads" ]

FROM rust:slim AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

WORKDIR /usr/src/app

COPY ./Cargo.lock .
COPY ./Cargo.toml .

WORKDIR /usr/src/app/crates
RUN cargo new server

WORKDIR /usr/src/app

COPY ./crates/server/Cargo.toml crates/server

RUN cargo fetch

COPY . .

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/server-ads ./
CMD [ "./server-ads" ]

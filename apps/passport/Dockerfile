FROM rust:bookworm as builder

WORKDIR /app

COPY --from=rust/libs /libs libs

RUN USER=root cargo new --bin apps/passport
COPY apps/passport/Cargo.toml apps/passport/Cargo.toml
RUN echo 'workspace = {members = ["apps/passport","libs/config","libs/db","libs/grpc","libs/nats", "libs/log","libs/utils"], resolver = "2"}'> Cargo.toml

RUN cargo build --release
RUN rm apps/passport/src/*.rs
COPY apps/passport/src apps/passport/src/

RUN rm ./target/release/deps/passport*
RUN rm ./target/release/deps/makoto_*
RUN rm ./target/release/deps/mafuyu_*
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt install -y openssl

WORKDIR /

COPY --from=builder /app/target/release/passport .

CMD ["./passport"]
FROM rust:bookworm as builder

WORKDIR /app

COPY --from=rust/libs /libs libs

RUN USER=root cargo new --bin apps/integrations
COPY apps/integrations/Cargo.toml apps/integrations/Cargo.toml
RUN echo 'workspace = {members = ["apps/integrations","libs/config","libs/db","libs/grpc","libs/nats", "libs/log","libs/utils"], resolver = "2"}'> Cargo.toml

RUN cargo build --release
RUN rm apps/integrations/src/*.rs
COPY apps/integrations/src apps/integrations/src/

RUN rm ./target/release/deps/integrations*
RUN rm ./target/release/deps/makoto_*
RUN rm ./target/release/deps/mafuyu_*
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt install -y openssl

WORKDIR /

COPY --from=builder /app/target/release/integrations .

CMD ["./integrations"]
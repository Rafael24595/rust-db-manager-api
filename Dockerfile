FROM rust:1.77.2

WORKDIR /rust-db-manager-api

COPY . .

RUN cargo build --release

ENTRYPOINT cargo run

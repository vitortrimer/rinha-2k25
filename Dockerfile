FROM rust:1.79-slim-buster AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release --locked --no-run

RUN rm src/main.rs

COPY src ./src

RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/my-rust-app /usr/local/bin/my-rust-app

EXPOSE 9999

CMD ["my-rust-app"]

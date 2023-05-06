FROM lukemathwalker/cargo-chef:latest-rust-1.59.0 AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recepie-path recepie.json

FROM chef AS builder
COPY --from=planner /app/recepie.json recepie.json
# Build and cache deps before source
RUN cargo chef cook --release --recepie-path recepie.json
# Build source
RUN cargo build --release --bin binary

FROM ubuntu:20.04 AS runtime
RUN apt-get update && apt install -y openssl ca-certificates
COPY --from=builder /app/target/release/binary .

ENTRYPOINT ["/binary"]

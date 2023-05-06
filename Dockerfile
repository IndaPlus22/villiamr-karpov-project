FROM lukemathwalker/cargo-chef:latest-rust-latest AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build and cache deps before source
RUN cargo chef cook --release --recipe-path recipe.json
# Build source
COPY . .
RUN cargo build --release --bin villiamr-karpov-project

FROM ubuntu:20.04 AS runtime
RUN apt-get update && apt install -y openssl ca-certificates
COPY --from=builder /app/target/release/villiamr-karpov-project .

ENTRYPOINT ["./villiamr-karpov-project"]

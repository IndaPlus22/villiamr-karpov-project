FROM rust:1.68 AS build

RUN cargo new --bin villiamr-karpov-project
WORKDIR /villiamr-karpov-project

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

RUN cargo build --release

FROM ubuntu:latest AS runtime

COPY --from=build /villiamr-karpov-project/target/release/villiamr-karpov-project .

ENTRYPOINT ["/villiamr-karpov-project"]

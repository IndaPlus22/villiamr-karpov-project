FROM rust:latest as builder

RUN USER=root cargo new --bin villiamr-karpov-project
WORKDIR ./villiamr-karpov-project
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN cargo build --release 

RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/villiamr-karpov-project*
RUN cargo build --release

FROM rust:latest

COPY --from=builder /todo/target/release/villiamr-karpov-project .

ENTRYPOINT["/villiamr-karpov-project"]

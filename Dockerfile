FROM rust:latest as builder

RUN USER=root cargo new --bin todo
WORKDIR ./todo
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN cargo build --release 

RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/todo*
RUN cargo build --release

FROM rust:latest

COPY --from=build /todo/target/release/todo .

ENTRYPOINT["/todo"]

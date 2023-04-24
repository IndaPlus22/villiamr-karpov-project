FROM rust:1.68 AS build

RUN cargo new --bin todo-actions
WORKDIR /todo-actions

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Compile and cache dependencies may prob if only few deps be removed later (here to not forget later)
RUN cargo build --release
RUN rm src/*.rs

# Copy source files
COPY ./src ./src

# Build source files 
RUN rm ./target/release/deps/todo-actions
RUN cargo build --release

FROM ubuntu:latest AS runtime

COPY --from=build /todo-actions/target/release/todo-actions .

ENTRYPOINT ["/todo-actions"]

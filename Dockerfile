FROM rust:1.68 AS build

RUN cargo new --bin villiamr-karpov-project
WORKDIR /villiamr-karpov-project

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Compile and cache dependencies may prob if only few deps be removed later (here to not forget later)
RUN cargo build --release
RUN rm src/*.rs

# Copy source files
COPY ./src ./src

RUN ls

# Build source files 
RUN rm ./target/release/deps/villiamr-karpov-project
RUN cargo build --release

FROM ubuntu:latest AS runtime

COPY --from=build /villiamr-karpov-project/target/release/villiamr-karpov-project .

ENTRYPOINT ["/villiamr-karpov-project"]

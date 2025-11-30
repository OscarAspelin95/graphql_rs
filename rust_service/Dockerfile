# Get cargo chef
FROM rust:1.90 AS chef
RUN cargo install cargo-chef
WORKDIR /usr/src/app

# Prepare recipe.
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build. This is the caching Docker layer.
FROM chef AS builder
COPY --from=planner /usr/src/app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin graphql_rs

FROM ubuntu:latest
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/graphql_rs /usr/local/bin
CMD ["/usr/local/bin/graphql_rs"]

FROM lukemathwalker/cargo-chef:latest-rust-1.72.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

FROM rust:1.72-slim AS template-rust
COPY --from=builder /app/target/release/quotes-bot /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/quotes-bot"]

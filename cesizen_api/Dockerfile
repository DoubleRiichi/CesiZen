# ---- Stage 1 : builder (identique au .test) ----
FROM rust:1.94-slim AS builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*
RUN cargo install sqlx-cli --no-default-features --features postgres
WORKDIR /app
COPY cesizen_api/Cargo.toml cesizen_api/Cargo.lock ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src
COPY cesizen_api/src ./src
RUN touch src/main.rs
RUN cargo build --release

# ---- Stage 2 : runtime allégé ----
FROM debian:trixie-slim AS runtime
RUN apt-get update && apt-get install -y libssl3 ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx
WORKDIR /app
COPY --from=builder /app/target/release/Cesizen ./api
COPY cesizen_api/migrations ./migrations
COPY cesizen_api/entrypoint.sh ./entrypoint.sh
RUN chmod +x entrypoint.sh
EXPOSE 8080
CMD ["./entrypoint.sh"]
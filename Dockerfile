FROM rust:1.83-slim AS builder
WORKDIR /app

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/servercompass-rust-actix-demo .
EXPOSE 8080
CMD ["./servercompass-rust-actix-demo"]

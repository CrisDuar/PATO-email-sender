FROM rust:latest AS builder
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev
WORKDIR /EmailService
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /EmailService
COPY --from=builder /EmailService/target/release/email-service .
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
EXPOSE 8000
CMD ["./email-service"]

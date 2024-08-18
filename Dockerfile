FROM rust:1.60 as builder
WORKDIR /app
COPY Cargo.toml .
COPY src ./src
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /app/target/release/apigateway /usr/local/bin/apigateway
EXPOSE 8080
CMD ["apigateway"]

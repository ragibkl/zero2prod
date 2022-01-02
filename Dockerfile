# Builder
FROM ekidd/rust-musl-builder as builder

COPY . .

ENV SQLX_OFFLINE true
RUN cargo build --release


# Runtime
FROM alpine:latest as runtime

WORKDIR /app
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/zero2prod zero2prod
COPY configuration configuration

ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]

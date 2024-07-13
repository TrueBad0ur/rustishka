FROM rust:1.79-bookworm as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM scratch

COPY --from=builder /app/target/release/rustishka /app/rustishka

CMD ["/app/rustishka"]
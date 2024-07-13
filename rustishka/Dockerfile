FROM --platform=linux/amd64 rust:1.79-bookworm as builder

WORKDIR /app

COPY . .

RUN apt update && apt install musl musl-dev musl-tools

RUN rustup target add x86_64-unknown-linux-musl

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM --platform=linux/amd64 alpine/socat:1.8.0.0

RUN ls -lah /

COPY --from=builder /app/target/release/rustishka /app/rustishka

ENTRYPOINT ["socat"]
CMD ["TCP4-LISTEN:31337,fork,reuseaddr,end-close", "EXEC:/app/rustishka"]

FROM rust:1.79-bookworm as builder

RUN apt update && apt install -y socat

WORKDIR /app

COPY . .

RUN cargo build --release

ENTRYPOINT ["socat"]
CMD ["TCP4-LISTEN:31337,fork,reuseaddr,end-close", "EXEC:/app/target/release/rustishka"]
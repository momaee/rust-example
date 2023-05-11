FROM rust:1.61.0

# Set destination for COPY
WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["./target/release/telegram-bot-rust"]
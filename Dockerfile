FROM rust:1.70

WORKDIR /usr/src/app
COPY . .

RUN cargo test

RUN cargo build --release

CMD ["./target/release/daily_challenge_notifier"]

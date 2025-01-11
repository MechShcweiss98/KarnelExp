FROM rust:latest

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

CMD ["echo", "Build complete: target/release/libroulette.a"]

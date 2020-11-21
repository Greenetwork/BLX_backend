FROM ubuntu:bionic

RUN apt update && apt install -y build-essential pkg-config libssl-dev curl

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && \
    source $HOME/.cargo/env && rustup default nightly

RUN mkdir /app

COPY . /app

WORKDIR /app

RUN cargo build --release

EXPOSE 8000

CMD ["./target/release/blx_backend"]

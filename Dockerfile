FROM ubuntu:xenial

RUN apt update && apt install -y build-essential pkg-config libssl-dev curl

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN . $HOME/.cargo/env && rustup default nightly

RUN mkdir /app

COPY . /app

WORKDIR /app

RUN . $HOME/.cargo/env && cargo build --release

EXPOSE 8000

CMD ["./target/release/blx_backend"]

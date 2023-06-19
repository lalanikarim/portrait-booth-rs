FROM rust:1.70-slim-bookworm
RUN apt update
RUN apt install -y vim libssl-dev pkg-config iputils-ping
RUN rustup update
RUN cargo install cargo-leptos
RUN rustup target add wasm32-unknown-unknown


FROM rust:1.70-slim-buster
RUN apt update
RUN apt install -y vim libssl-dev pkg-config
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos


FROM ubuntu:16.04 as builder
#FROM rust:1.70-slim-bookworm as builder
RUN apt update; \
    apt install -y vim libssl-dev pkg-config iputils-ping musl-dev curl;
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN set -eux; . $HOME/.cargo/env && cargo install --version 0.1.11 cargo-leptos; 
RUN mkdir -p /app;
WORKDIR /app
COPY . .
RUN set -eux; . $HOME/.cargo/env && cargo clean;
RUN set -eux; . $HOME/.cargo/env && rustup target add wasm32-unknown-unknown;
RUN set -eux; . $HOME/.cargo/env && rustup target add x86_64-unknown-linux-musl;
ENV SQLX_OFFLINE=true
RUN set -eux; . $HOME/.cargo/env && cargo leptos build --release -vv;
RUN ls -l /app/target;

FROM ubuntu:16.04 as runner
#FROM rust:1.70-slim-bookworm as runner
COPY --from=builder /app/target/server/release/portrait-booth /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/
COPY --from=builder /app/start.sh /app/
COPY --from=builder /app/stop.sh /app/
WORKDIR /app
ENV RUST_LOG="info"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080
CMD ["sh","-c","/app/$LEPTOS_OUTPUT_NAME"]


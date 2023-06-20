FROM rust:alpine3.18
RUN apk add bash vim musl-dev openssl-dev pkgconfig ca-certificates openssl-libs-static libc-dev
RUN cargo install --version 0.1.11 cargo-leptos;
RUN mkdir -p /app;
WORKDIR /app
COPY . .
RUN set -eux; cargo clean;
RUN set -eux; rustup target add wasm32-unknown-unknown;
RUN set -eux; rustup target add x86_64-unknown-linux-musl;
ENV SQLX_OFFLINE=true
RUN set -eux; cargo leptos build --release -vv;
RUN ls -l /app/target;

FROM rust:alpine3.18 as runner
COPY --from=builder /app/target/server/release/portrait-booth /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/
WORKDIR /app
ENV RUST_LOG="info"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080
CMD ["sh","-c","/app/$LEPTOS_OUTPUT_NAME"]

#!/usr/bin/env bash
set -e
cargo clean
TARGET_CC=x86_64-linux-musl-gcc LEPTOS_BIN_TARGET_TRIPLE=x86_64-unknown-linux-musl SQLX_OFFLINE=true cargo leptos build -r
rm -r deploy/*
mkdir deploy/app
cp target/server/x86_64-unknown-linux-musl/release/portrait-booth deploy/app/
cp -r target/site deploy/app/
cp Cargo.toml deploy/app/
cp {start,stop}.sh deploy/app/
chmod +x deploy/app/*.sh 
cd deploy
tar cvf app.tar app
gzip app.tar

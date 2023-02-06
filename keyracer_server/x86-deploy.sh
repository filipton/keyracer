#!/bin/bash

rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu

docker buildx build --platform linux/amd64 --tag filipton/keyracer-server:latest --push -f x86.Dockerfile .

#!/bin/bash

rustup target add aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu

buildx build --platform linux/arm64 --tag filipton/keyracer-server:latest --load -f arm.Dockerfile .

docker image push filipton/keyracer-server:latest
docker image tag filipton/keyracer-server:latest "filipton/keyracer-server:$1"
docker image push "filipton/keyracer-server:$1"

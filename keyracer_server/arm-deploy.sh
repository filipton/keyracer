#!/bin/bash

rustup target add aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu

buildx build --platform linux/arm64 --tag filipton/keyracer-server:latest --push -f arm.Dockerfile .

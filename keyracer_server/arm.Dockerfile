FROM debian:buster-slim

COPY ./target/aarch64-unknown-linux-gnu/release/keyracer_server /usr/local/bin/keyracer_server
CMD ["keyracer_server"]

FROM debian:buster-slim
WORKDIR /app
COPY ./words_list.txt .
COPY ./quotes.json .

COPY ./target/aarch64-unknown-linux-gnu/release/keyracer_server /usr/local/bin/keyracer_server
CMD ["keyracer_server"]

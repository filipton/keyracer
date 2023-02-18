FROM debian:backworm-slim
WORKDIR /app
COPY ./words_list.txt .
COPY ./quotes.json .

RUN apt update -y && apt install libssl-dev -y

COPY ./target/aarch64-unknown-linux-gnu/release/keyracer_server /usr/local/bin/keyracer_server
CMD ["keyracer_server"]

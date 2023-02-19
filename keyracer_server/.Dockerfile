FROM debian:bookworm-slim
WORKDIR /app
COPY ./words_list.txt .
COPY ./quotes.json .

RUN apt update -y && apt install libssl-dev -y

COPY ./target/x86_64-unknown-linux-gnu/release/keyracer_server /usr/local/bin/keyracer_server
CMD ["keyracer_server"]

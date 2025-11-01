FROM rust:alpine

WORKDIR /0-shell
COPY . .

RUN cargo build

CMD ["cargo", "r"]

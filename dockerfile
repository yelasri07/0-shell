FROM rust:alpine

WORKDIR /0-shell
COPY . .

CMD ["cargo", "r"]

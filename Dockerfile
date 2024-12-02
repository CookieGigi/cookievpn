FROM rust:latest

WORKDIR /usr/src/cookievpn
COPY . .

RUN cargo install --path .

CMD ["sh"]


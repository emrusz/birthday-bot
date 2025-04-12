FROM rust:1.86.0

WORKDIR /usr/src/birthday-bot
COPY . .

RUN cargo install --path .

CMD ["birthday-bot"]

FROM rust:1.42

WORKDIR /usr/src/weather-bot
COPY . .

RUN cargo install --path .

CMD ["weather-bot"]
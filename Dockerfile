FROM rust:1.31

WORKDIR /usr/src/fan-quiz-juniper
COPY . .

RUN cargo install --path .

CMD ["fan-quiz-juniper"]
FROM rust:1.69-buster as budilder

WORKDIR /app

ARG DATABASE_URL

ENV DATABASE_URL=$DATABASE_URL

COPY . .

RUN cargo build --release


FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=budilder /app/target/release/rust-crud-api .

CMD ["./rust-crud-api"]
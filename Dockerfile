FROM rust:1.74 as builder

WORKDIR /usr/src/web_voyager

COPY . .

# RUN cargo install --path .

RUN cargo build --release --verbose

FROM ubuntu:jammy as environment

WORKDIR /usr/web_voyager

COPY --from=builder /usr/src/web_voyager/target/release/web-voyager-judger ./web-voyager-judger
COPY ./.env.production .

EXPOSE 8080

ENTRYPOINT ./web-voyager-judger

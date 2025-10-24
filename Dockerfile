FROM rust:1.90.0 AS build

WORKDIR /app

COPY . .

RUN cargo build --release

FROM cgr.dev/chainguard/glibc-dynamic:latest

WORKDIR /usr/src/app

COPY --from=build /app/target/release/eventshuffle-rs .

CMD [ "./eventshuffle-rs" ]

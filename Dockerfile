FROM rust:latest as builder
RUN apt-get update && apt-get install -y libpq-dev cmake

WORKDIR /app

COPY . . 

RUN cargo build --release

# Production stage
FROM debian:buster-slim

WORKDIR /usr/local/bin
FROM rust:latest
RUN apt-get update && apt-get install -y libpq-dev curl
COPY --from=builder /app/target/release/stocks-consumer .

CMD ["./stocks-consumer"]

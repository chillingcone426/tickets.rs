FROM rust:1-buster

RUN apt-get install -y apt-transport-https
RUN apt-get update && apt-get -y upgrade && apt-get -y install python3 openssl libssl-dev ca-certificates cmake

RUN mkdir -p /tmp/compile
WORKDIR /tmp/compile

COPY . .

ENV RUSTFLAGS="-C target-cpu=core2 -C target-feature=-avx2,-avx512f"
RUN cargo clean && cargo build --release

# Prod container
FROM debian:buster

RUN apt-get update && apt-get -y upgrade && apt-get -y install openssl libssl-dev ca-certificates

COPY --from=0 /tmp/compile/target/release/http-gateway /srv/gateway/http-gateway
RUN chmod +x /srv/gateway/http-gateway

RUN useradd -m container
USER container
WORKDIR /srv/gateway

CMD ["/srv/gateway/http-gateway"]

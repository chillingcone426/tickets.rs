# Build stage
FROM rust:1-buster AS builder

# Install dependencies
RUN apt-get update && apt-get -y upgrade && \
    apt-get -y install python3 openssl libssl-dev ca-certificates cmake

# Set up working directory
RUN mkdir -p /tmp/compile
WORKDIR /tmp/compile

# Copy source code
COPY . .

# Force Rust to avoid AVX2 & AVX512
ENV RUSTFLAGS="-C target-cpu=core2 -C target-feature=-avx2,-avx512f"

# Ensure dependencies are rebuilt with these flags
RUN cargo clean && cargo update && cargo fetch
RUN cargo build --release

# Production container
FROM debian:buster AS prod

# Install runtime dependencies
RUN apt-get update && apt-get -y upgrade && apt-get -y install openssl libssl-dev ca-certificates

# Copy the compiled binary
COPY --from=builder /tmp/compile/target/release/http-gateway /srv/gateway/http-gateway

# Set executable permissions
RUN chmod +x /srv/gateway/http-gateway

# Create a non-root user
RUN useradd -m container
USER container

# Set working directory
WORKDIR /srv/gateway

# Run the application
CMD ["/srv/gateway/http-gateway"]

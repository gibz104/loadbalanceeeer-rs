# Use the official Rust image as a base
FROM rust:latest as builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml file
COPY Cargo.toml ./

# Copy the actual source code
COPY src src

# Update the dependencies
RUN cargo update

# Build the binary
RUN cargo build --release

# Use a lightweight base image for the runtime
FROM debian:stable-slim

# Install necessary runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates libssl-dev netcat && \
    rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/lb /usr/local/bin/

# Copy the wait-for-tor.sh script from the builder stage
COPY --from=builder /usr/src/app/src/wait-for-tor.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/wait-for-tor.sh

# Expose the WebSocket proxy port
EXPOSE 9545

# Wait for socks proxy to be available
CMD ["wait-for-tor.sh", "tor-socks-proxy:9150", "lb"]

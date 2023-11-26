# Start from the latest Rust base image
FROM rust:latest as builder

# Set the current working directory in the Docker image
WORKDIR /usr/src/app

# Copy over your manifest
COPY ./Cargo.toml ./Cargo.toml

# Copy your source tree
COPY ./src ./src

# Build for release. 
# This will also download and compile your dependencies
RUN cargo build --release --bin dns-resolver

# Our second stage, that will be the final image
FROM ubuntu:latest

# Install SSL certificates and clean up in one RUN command
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the builder stage
COPY --from=builder /usr/src/app/target/release/dns-resolver .

# Expose port 3030
EXPOSE 3030

# Set the startup command to run your binary
CMD ["./dns-resolver"]
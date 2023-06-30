# Use the Rust official image as the base
FROM rust:1.55 AS builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    openssl \
    libssl-dev

# Set the environment variable for backtraces
ENV RUST_BACKTRACE=1

# Create a new directory for the application
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Build and cache dependencies
RUN cargo build

# Copy the source code to the container
COPY src ./src

# Build the application
RUN cargo build

# Expose the port the server listens on
EXPOSE 8000

# Set the entry point command to start the server
CMD ["cargo", "run"]

# Build stage
FROM rust:1.70 as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    cmake \
    pkg-config \
    libssl-dev \
    git \
    clang

WORKDIR /usr/src/laniakea
COPY . .

# Build the project
RUN cargo build --release

# Runtime stage
FROM nvidia/cuda:11.8.0-base-ubuntu22.04

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl1.1 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /usr/src/laniakea/target/release/laniakea /usr/local/bin/

# Create app directory
WORKDIR /app

# Copy necessary files
COPY --from=builder /usr/src/laniakea/config ./config
COPY --from=builder /usr/src/laniakea/.env.example ./.env

# Expose ports
EXPOSE 3000

# Set environment variables
ENV RUST_LOG=info

# Run the binary
CMD ["laniakea"]
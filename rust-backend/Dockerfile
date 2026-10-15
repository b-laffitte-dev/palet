# Dockerfile for Palet Vendéen Backend (Rust)

# Build stage
FROM rust:slim AS builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy source code
WORKDIR /app
COPY . .

# Build the application
RUN cargo build --release --bin palet-backend

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from build stage
COPY --from=builder /app/target/release/palet-backend /usr/local/bin/palet-backend

# Copy configuration
COPY --from=builder /app/rust-backend/config /app/config

# Create app directory
WORKDIR /app

# Expose port
EXPOSE 3000

# Set environment variables
ENV RUST_LOG=info
ENV PALET__ENVIRONMENT=production

# Run the application
CMD ["palet-backend"]

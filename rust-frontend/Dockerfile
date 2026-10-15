# Dockerfile for Palet Vendéen Frontend (Rust/Yew)

# Build stage
FROM rust:slim AS builder

# Install wasm-pack
RUN cargo install wasm-pack

# Install nodejs for npm dependencies (if needed)
RUN apt-get update && apt-get install -y \
    curl \
    && curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Copy source code
WORKDIR /app
COPY . .

# Build the wasm application
RUN cd rust-frontend && wasm-pack build --target web --out-name palet_frontend --out-dir ./pkg

# Runtime stage using nginx
FROM nginx:alpine

# Copy built files
COPY --from=builder /app/rust-frontend/pkg /usr/share/nginx/html/pkg
COPY --from=builder /app/rust-frontend/static /usr/share/nginx/html/static

# Copy index.html
COPY index.html /usr/share/nginx/html/index.html

# Copy nginx configuration
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Expose port
EXPOSE 80

# Run nginx
CMD ["nginx", "-g", "daemon off;"]

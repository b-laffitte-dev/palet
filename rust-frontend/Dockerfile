# Dockerfile for Palet Vendéen Frontend (Rust/Yew)

# Build stage
FROM rust:slim AS builder

# Install wasm-pack
RUN cargo install wasm-pack

# Add the wasm compilation target
RUN rustup target add wasm32-unknown-unknown

# Copy source code
WORKDIR /app
COPY . .

# Build the wasm application
RUN cd rust-frontend && wasm-pack build --target web --out-name palet_frontend --out-dir ./pkg

# Runtime stage using nginx
FROM nginx:alpine

# Copy built files
COPY --from=builder /app/rust-frontend/pkg /usr/share/nginx/html/pkg

# Copy index.html
COPY rust-frontend/index.html /usr/share/nginx/html/index.html

# Copy nginx configuration
COPY rust-frontend/nginx.conf /etc/nginx/conf.d/default.conf

# Expose port
EXPOSE 80

# Run nginx
CMD ["nginx", "-g", "daemon off;"]

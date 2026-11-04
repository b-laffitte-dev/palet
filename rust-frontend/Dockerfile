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

# Copy design styles (ported from origin/main web/src/styles)
COPY rust-frontend/styles /usr/share/nginx/html/styles

# Copy nginx configuration
COPY rust-frontend/nginx.conf /etc/nginx/conf.d/default.conf

# Ensure nginx (worker user) can read the served files regardless of host file modes
RUN chmod -R a+rX /usr/share/nginx/html

# Expose port
EXPOSE 80

# Run nginx
CMD ["nginx", "-g", "daemon off;"]

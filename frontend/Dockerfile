# Build stage
FROM rust:alpine AS builder

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    pkgconfig \
    perl \
    make

# Install wasm target
RUN rustup target add wasm32-unknown-unknown

# Install trunk from binary
RUN wget -qO- https://github.com/trunk-rs/trunk/releases/latest/download/trunk-x86_64-unknown-linux-musl.tar.gz | tar -xzf- -C /usr/local/bin

# Create app directory
WORKDIR /app

# Copy all files
COPY . .

# Build the application
RUN trunk build --release

# Runtime stage
FROM nginx:alpine

# Copy built files from builder
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy nginx configuration
COPY nginx.conf /etc/nginx/nginx.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
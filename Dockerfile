# Multi-stage Dockerfile for Sparrow 2022 Rust + WebAssembly project

# Stage 1: Build Rust WebAssembly library
FROM rust:1.62.0 as rust-builder

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
ENV PATH="/root/.cargo/bin:${PATH}"

# Set working directory
WORKDIR /app

# Copy Rust toolchain configuration
COPY rust-toolchain.toml .

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./
COPY libs/ libs/

# Build the WebAssembly library
RUN cd libs/simulation-wasm && wasm-pack build --target bundler --out-dir pkg

# Stage 2: Build Node.js client
FROM node:16-alpine as node-builder

# Set working directory
WORKDIR /app

# Copy client source code
COPY client/ ./

# Create the correct directory structure for the client
RUN mkdir -p libs/simulation-wasm

# Copy the built WASM library from rust-builder stage
COPY --from=rust-builder /app/libs/simulation-wasm/pkg ./libs/simulation-wasm/pkg

# Update package.json to use the correct path
RUN sed -i 's|"lib-simulation-wasm": "file:../libs/simulation-wasm/pkg"|"lib-simulation-wasm": "file:./libs/simulation-wasm/pkg"|' package.json

# Install dependencies (after copying the WASM library)
RUN npm install

# Build the client
RUN npm run build

# Stage 3: Production server
FROM nginx:alpine

# Copy built client files
COPY --from=node-builder /app/dist /usr/share/nginx/html

# Copy nginx configuration
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Expose port
EXPOSE 80

# Start nginx
CMD ["nginx", "-g", "daemon off;"]

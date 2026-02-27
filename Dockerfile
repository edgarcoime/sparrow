FROM debian:bookworm-slim AS build

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    git \
    build-essential \
    pkg-config \
    clang \
    xz-utils \
    && rm -rf /var/lib/apt/lists/*

SHELL ["/bin/bash", "-o", "pipefail", "-c"]
ENV MISE_DATA_DIR="/mise"
ENV MISE_CONFIG_DIR="/mise"
ENV MISE_CACHE_DIR="/mise/cache"
ENV MISE_INSTALL_PATH="/usr/local/bin/mise"
ENV MISE_EXPERIMENTAL="1"
ENV PATH="/mise/shims:${PATH}"

RUN curl https://mise.run | sh

WORKDIR /app

COPY . .

# Trust mise config files in this repo (required in non-interactive/container builds).
RUN mise trust /app/mise.toml \
    && mise trust /app/client/mise.toml \
    && mise trust /app/simple/mise.toml

# Install tools declared in mise.toml and build artifacts (WASM + Astro static output).
RUN mise install
RUN rustup target add wasm32-unknown-unknown
RUN mise run build

FROM nginx:1.27-alpine AS runner

COPY --from=build /app/client/dist /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]

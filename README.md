# Sparrows

Simulation of life and evolution powered by Neural Networks and Genetic algorithms.

## Docker

```bash
# Build
docker build -t sparrow-client:latest .
docker build -t <dockerhub-user>/sparrow-client:latest .
# run
docker run --rm -p 3000:80 sparrow-client:latest
# In production
docker run -d --name sparrow --restart unless-stopped -p 80:80 sparrow-client:latest
# Upload to docker
docker push <dockerhub-user>/sparrow-client:latest
```

## Multiarch Compilation

Use this when deploying to ARM hosts (for example `aarch64`) or mixed fleets.

```bash
# One-time setup (or reuse an existing builder)
docker buildx create --use --name multiarch || docker buildx use multiarch

# Build and push ARM64 only
docker buildx build \
  --platform linux/arm64 \
  -t <dockerhub-user>/sparrow-client:latest \
  --push .

# Build and push multi-arch (AMD64 + ARM64)
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  -t <dockerhub-user>/sparrow-client:latest \
  --push .

# with other tags for rollback
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  -t <dockerhub-user>/sparrow-client:latest \
  -t <dockerhub-user>/sparrow-client:v0.0.2 \
  --push .
```

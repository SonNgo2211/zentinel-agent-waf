# syntax=docker/dockerfile:1.4

# Zentinel WAF Agent Container Image
#
# Targets:
#   - prebuilt: For CI with pre-built binaries

################################################################################
# Pre-built binary stage (for CI builds)
################################################################################
# Build arguments
ARG RUST_VERSION=1.88
ARG DEBIAN_VARIANT=slim-bookworm

################################################################################
# Build stage - compiles the Rust binary with optimizations
################################################################################
FROM rust:${RUST_VERSION}-${DEBIAN_VARIANT} AS builder

# Install build dependencies (only what's needed for compilation)
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev \
        protobuf-compiler \
        cmake \
        build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifest files first for better layer caching
COPY Cargo.toml ./
COPY src/ src/
COPY benches/ benches/
COPY crates/ crates/
# Build dependencies only (this layer is cached)
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12:nonroot

COPY --from=builder /app/target/release/zentinel-waf-agent /zentinel-waf-agent

LABEL org.opencontainers.image.title="Zentinel WAF Agent" \
      org.opencontainers.image.description="Zentinel WAF Agent for Zentinel reverse proxy" \
      org.opencontainers.image.vendor="Raskell" \
      org.opencontainers.image.source="https://github.com/zentinelproxy/zentinel-agent-waf"

ENV RUST_LOG=info,zentinel_waf_agent=debug \
    SOCKET_PATH=/var/run/zentinel/waf.sock

USER nonroot:nonroot

ENTRYPOINT ["/zentinel-waf-agent"]

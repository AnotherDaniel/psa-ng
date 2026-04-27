# Copyright (C) 2026 psa-ng project contributors.
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, version 3.
#
# SPDX-FileType: SOURCE
# SPDX-FileCopyrightText: 2026 psa-ng project contributors
# SPDX-License-Identifier: GPL-3.0-only

# [impl->req~container-image~1]
# ── Build stage ───────────────────────────────────────────────────────
FROM rust:1.95-slim AS builder

WORKDIR /build

# Install build dependencies for rusqlite (bundled SQLite) and reqwest (OpenSSL)
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests first for dependency caching
COPY Cargo.toml Cargo.lock ./
COPY psa-api/Cargo.toml psa-api/Cargo.toml
COPY psa-web/Cargo.toml psa-web/Cargo.toml

# Create stub lib/main so cargo can resolve the workspace and cache deps
RUN mkdir -p psa-api/src psa-web/src \
    && echo 'pub mod auth; pub mod client; pub mod config; pub mod error; pub mod models;' > psa-api/src/lib.rs \
    && touch psa-api/src/auth.rs psa-api/src/client.rs psa-api/src/config.rs psa-api/src/error.rs psa-api/src/models.rs \
    && echo 'fn main() {}' > psa-web/src/main.rs \
    && cargo build --release --package psa-web 2>/dev/null || true

# Copy real source and build
COPY psa-api/src psa-api/src
COPY psa-web/src psa-web/src
RUN touch psa-api/src/lib.rs psa-web/src/main.rs \
    && cargo build --release --package psa-web

# ── Runtime stage ─────────────────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd --gid 1000 psa \
    && useradd --uid 1000 --gid psa --shell /bin/false psa \
    && mkdir -p /app/data \
    && chown -R psa:psa /app

WORKDIR /app

COPY --from=builder /build/target/release/psa-web /app/psa-web

USER psa

EXPOSE 5000

VOLUME ["/app/data"]

ENTRYPOINT ["/app/psa-web"]
CMD ["/app/config.toml"]

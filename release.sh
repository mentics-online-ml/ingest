#!/bin/bash

# RUSTFLAGS='-C link-arg=-s -Zlocation-detail=none' \
#     cargo +nightly build \
#         -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort \
#         --target x86_64-unknown-linux-gnu \
#         --release \
#         -Z unstable-options \
#         --out-dir target

RUSTFLAGS='-C link-arg=-s' cargo build --release --target aarch64-unknown-linux-musl

docker build -t ghcr.io/mentics-online-ml/ingest:latest .
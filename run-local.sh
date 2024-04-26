#!/bin/bash
set -e

source setenv.sh

if ! docker ps | grep -q redpanda; then
    ./redpanda/run.sh
fi

export RUST_BACKTRACE=1
cargo run

# docker run -e TRADIER_API_KEY=`cat ~/.tradier_api_key` ghcr.io/mentics-ml-demo/ingest:latest
#!/bin/bash
set -e
LOCAL_DIR="$(dirname "${BASH_SOURCE[0]}")"
BASE_DIR=$(realpath "$LOCAL_DIR/..")
source "$LOCAL_DIR"/setenv.sh

if ! docker ps | grep -q redpanda; then
    "$LOCAL_DIR"/redpanda/run.sh
fi

cd "$BASE_DIR"

RUST_BACKTRACE=1 cargo run

# docker run -e TRADIER_API_KEY=`cat ~/.tradier_api_key` ghcr.io/mentics-ml-demo/ingest:latest
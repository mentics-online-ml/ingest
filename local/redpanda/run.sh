#!/usr/bin/env bash
cd "$(dirname "${BASH_SOURCE[0]}")" || exit

# https://docs.redpanda.com/current/get-started/quick-start/

docker compose up -d
# TODO: wait for docker containers to be ready
# docker exec -it redpanda-0 rpk topic create raw event label

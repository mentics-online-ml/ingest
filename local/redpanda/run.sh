#!/usr/bin/env bash

# https://docs.redpanda.com/current/get-started/quick-start/

docker compose up -d

docker exec -it redpanda-0 rpk topic create raw
docker exec -it redpanda-0 rpk topic create features
docker exec -it redpanda-0 rpk topic create actual
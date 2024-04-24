#!/usr/bin/env bash
docker compose up -d

docker exec -it redpanda-0 rpk topic create raw
docker exec -it redpanda-0 rpk topic create features
docker exec -it redpanda-0 rpk topic create actual
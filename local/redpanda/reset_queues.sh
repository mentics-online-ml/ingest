#!/usr/bin/env bash

docker exec -it redpanda-0 rpk topic delete raw event label
docker exec -it redpanda-0 rpk topic create raw event label

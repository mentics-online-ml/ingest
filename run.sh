#!/bin/bash
docker run -e TRADIER_API_KEY=`cat ~/.tradier_api_key` ghcr.io/mentics-ml-demo/ingest:latest
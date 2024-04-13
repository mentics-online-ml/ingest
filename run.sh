#!/bin/bash

docker build .

docker run -e TRADIER_API_KEY=`cat ~/.tradier_api_key` mentics/ingest
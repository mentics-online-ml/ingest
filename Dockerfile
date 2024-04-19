FROM scratch
LABEL org.opencontainers.image.source="https://github.com/mentics-ml-demo/ingest"

COPY target/release/ingest /

CMD ["/ingest"]
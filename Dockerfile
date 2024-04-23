FROM scratch
LABEL org.opencontainers.image.source="https://github.com/mentics-ml-demo/ingest"

COPY target/aarch64-unknown-linux-musl/release/ingest /

CMD ["/ingest"]
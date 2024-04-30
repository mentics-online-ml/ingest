FROM scratch
LABEL org.opencontainers.image.source="https://github.com/mentics-online-ml/ingest"

COPY target/aarch64-unknown-linux-musl/release/ingest /

CMD ["/ingest"]
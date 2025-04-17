# syntax=docker/dockerfile:1
FROM rust:1.86.0-bullseye AS builder

ADD . .

ENV RUSTFLAGS="-C target-cpu=haswell -C opt-level=3"

RUN cargo build --release

FROM rust:1.86.0-slim-bullseye AS runner

COPY --from=builder --chown=65534 /target/release/portfolio-manager /usr/local/bin

ENV RUST_BACKTRACE=full

ENV PORT=${PORT:-8080}

EXPOSE $PORT

USER 65534

CMD ["/usr/local/bin/portfolio-manager"]
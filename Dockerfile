FROM rustlang/rust:nightly-trixie AS builder

RUN apt-get update -y && \
    apt-get install -y --no-install-recommends \
        clang \
        binaryen \
        brotli \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-leptos --locked

RUN rustup target add wasm32-unknown-unknown

RUN mkdir -p /app
WORKDIR /app
COPY . .

RUN cargo leptos build --release -vv

RUN wasm-opt -Oz --strip-debug \
    target/site/pkg/portfolio-ssr.wasm \
    -o target/site/pkg/portfolio-ssr.wasm

RUN find target/site/pkg -type f \( -name "*.wasm" -o -name "*.js" -o -name "*.css" \) \
    -exec brotli -q 11 -k -f {} \;
RUN find target/site -type f \( -name "*.webp" -o -name "*.svg" -o -name "*.js" \) \
    -exec brotli -q 11 -k -f {} \;

FROM debian:trixie-slim AS runtime

WORKDIR /app

RUN apt-get update -y && \
    apt-get install -y --no-install-recommends \
        openssl \
        curl \
        ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/portfolio-ssr /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"

EXPOSE 8080

CMD ["/app/portfolio-ssr"]
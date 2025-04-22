# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-alpine as builder

RUN apk update && \
    apk add --no-cache \
      bash \
      curl \
      npm \
      libc-dev \
      build-base \
      clang \
      llvm-dev \
      lld \
      binaryen \
      wasi-sdk

RUN apk add --no-cache wget \
  && wget https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-19/wasi-sdk-19.0-linux.tar.gz \
  && tar -xzf wasi-sdk-19.0-linux.tar.gz -C /opt \
  && rm wasi-sdk-19.0-linux.tar.gz

ENV WASI_SDK_PATH=/opt/wasi-sdk-19.0
ENV CC_wasm32_unknown_unknown="${WASI_SDK_PATH}/bin/clang --target=wasm32-unknown-unknown --sysroot=${WASI_SDK_PATH}/share/wasi-sysroot"

RUN npm install -g sass

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/download/v0.2.26/cargo-leptos-installer.sh | sh

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
COPY . .

RUN cargo leptos build --release -vv

FROM rustlang/rust:nightly-alpine as runner

WORKDIR /app

COPY --from=builder /work/target/release/regalk /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/
COPY --from=builder /work/rss.xml /app/
COPY --from=builder /work/blogs /app/blogs

EXPOSE 3000
ENV LEPTOS_SITE_ROOT=./site
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"

CMD ["/app/regalk"]

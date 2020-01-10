FROM rust:1.40

ARG rust_version="1.40.0"

RUN set -x && \
  apt-get update && \
  apt-get install -y gcc-arm-linux-gnueabihf

RUN set -x && \
  rustup target add --toolchain ${rust_version} armv7-unknown-linux-musleabihf && \
  rustup target add --toolchain ${rust_version} x86_64-unknown-linux-musl && \
  rustup component add rustfmt --toolchain ${rust_version}-x86_64-unknown-linux-gnu

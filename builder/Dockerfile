FROM rust:1.42

ARG rust_version="1.42.0"

ENV WSB_WORKSPACE="/workspace"

RUN set -x && \
  apt-get update && \
  apt-get install -y \
    gcc-arm-linux-gnueabihf \
    jq \
    musl-tools \
    clang \
    cmake \
    tree

RUN set -x && \
  rustup target add --toolchain ${rust_version} armv7-unknown-linux-musleabihf && \
  rustup target add --toolchain ${rust_version} x86_64-unknown-linux-musl && \
  rustup target add --toolchain ${rust_version} x86_64-apple-darwin && \
  rustup component add rustfmt --toolchain ${rust_version}-x86_64-unknown-linux-gnu && \
  rustup component add clippy --toolchain ${rust_version}-x86_64-unknown-linux-gnu

WORKDIR ${WSB_WORKSPACE}
RUN set -x && \
  git clone https://github.com/tpoechtrager/osxcross.git

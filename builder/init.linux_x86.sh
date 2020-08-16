#!/usr/bin/env bash

set -xue

# todo: share this value with init.sh and Dockerfile
PLUS_RUST_VERSION="1.42.0"

main() {
#  rustup toolchain install "${PLUS_RUST_VERSION}"-x86_64-unknown-linux-gnu
#  rustup toolchain install "${PLUS_RUST_VERSION}"-x86_64-unknown-linux-musl
  rustup target add --toolchain "${PLUS_RUST_VERSION}" x86_64-unknown-linux-musl
  echo "done!"
}

main

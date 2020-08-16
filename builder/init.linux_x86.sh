#!/usr/bin/env bash

set -xue

main() {
  rustup toolchain install "${PLUS_RUST_VERSION}"-x86_64-unknown-linux-gnu
  rustup target add --toolchain "${PLUS_RUST_VERSION}" x86_64-unknown-linux-musl
  echo "done!"
}

main

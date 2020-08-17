#!/usr/bin/env bash

set -xue

. ./builder/init/env.sh

main() {
  rustup toolchain install "${PLUS_RUST_VERSION}"-x86_64-unknown-linux-gnu
#  rustup toolchain install "${PLUS_RUST_VERSION}"-x86_64-unknown-linux-musl
  rustup target add --toolchain "${PLUS_RUST_VERSION}" x86_64-unknown-linux-musl
  rustup override set "${PLUS_RUST_VERSION}"

  echo "done!"
}

main

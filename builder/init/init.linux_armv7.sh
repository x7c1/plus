#!/usr/bin/env bash

set -xue

. ./builder/init/setup.sh

main() {
#  rustup toolchain install "${PLUS_RUST_VERSION}"-x86_64-unknown-linux-gnu
  rustup target add --toolchain "${PLUS_RUST_VERSION}" armv7-unknown-linux-musleabihf
  echo "done!"
}

main

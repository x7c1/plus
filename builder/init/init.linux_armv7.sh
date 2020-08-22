#!/usr/bin/env bash

set -xue

apt-get update
apt-get install -y \
  gcc-arm-linux-gnueabihf

rustup target add --toolchain "${PLUS_RUST_VERSION}" armv7-unknown-linux-musleabihf

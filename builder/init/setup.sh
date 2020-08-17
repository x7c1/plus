#!/usr/bin/env bash

set -xue

PLUS_RUST_VERSION="1.42.0"
export PLUS_RUST_VERSION

rustup toolchain install "${PLUS_RUST_VERSION}"-x86_64-unknown-linux-gnu
rustup override set "${PLUS_RUST_VERSION}"

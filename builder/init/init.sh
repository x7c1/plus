#!/usr/bin/env bash

set -xue

rustup toolchain install "${PLUS_RUST_VERSION}"-x86_64-unknown-linux-gnu
rustup override set "${PLUS_RUST_VERSION}"

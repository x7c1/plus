#!/usr/bin/env bash

set -xue

rustup component add rustfmt --toolchain "$PLUS_RUST_VERSION"-x86_64-unknown-linux-gnu
rustup component add clippy --toolchain "$PLUS_RUST_VERSION"-x86_64-unknown-linux-gnu

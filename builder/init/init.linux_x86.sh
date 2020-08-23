#!/usr/bin/env bash

set -xue

rustup target add --toolchain "${PLUS_RUST_VERSION}" x86_64-unknown-linux-musl

#!/usr/bin/env bash

set -xue

rustup target add --toolchain "${PLUS_RUST_VERSION}" armv7-unknown-linux-musleabihf

#!/usr/bin/env bash

set -xue

. ./builder/init/setup.sh

rustup target add --toolchain "${PLUS_RUST_VERSION}" armv7-unknown-linux-musleabihf

#!/usr/bin/env bash

set -xue

PLUS_RUST_VERSION="1.42.0"
export PLUS_RUST_VERSION

#if command -v rustc; then
#  echo "rust already installed."
#else
#  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /usr/bin/rustup-init
#  chmod +x /usr/bin/rustup-init
#  rustup-init -y
#  . $HOME/.cargo/env
#fi

rustup toolchain install "${PLUS_RUST_VERSION}"-x86_64-unknown-linux-gnu
rustup override set "${PLUS_RUST_VERSION}"

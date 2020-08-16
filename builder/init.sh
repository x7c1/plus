#!/usr/bin/env bash

set -xue

rust_version="1.42.0"

# todo: rename WSB
workspace="$WSB_WORKSPACE"

main() {
  setup_rust

  cd "$workspace"
  setup_other_repositories

  echo "done!"
}

setup_rust() {
  rustup target add --toolchain ${rust_version} armv7-unknown-linux-musleabihf
  rustup target add --toolchain ${rust_version} x86_64-unknown-linux-musl
  rustup target add --toolchain ${rust_version} x86_64-apple-darwin
  rustup component add rustfmt --toolchain ${rust_version}-x86_64-unknown-linux-gnu
  rustup component add clippy --toolchain ${rust_version}-x86_64-unknown-linux-gnu
}

setup_other_repositories() {
  git clone https://github.com/tpoechtrager/osxcross.git
}

main

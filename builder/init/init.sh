#!/usr/bin/env bash

set -xue

PLUS_RUST_VERSION="1.42.0"
export PLUS_RUST_VERSION

# todo: rename WSB
workspace="$WSB_WORKSPACE"

main() {
  setup_rust

  cd "$workspace"
  setup_other_repositories

  echo "done!"
}

setup_rust() {
#  if command -v rustc; then
#    echo "rust already installed."
#  else
#    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
#  fi

  rustup toolchain install ${PLUS_RUST_VERSION}-x86_64-unknown-linux-gnu

#  "$workspace"/init.linux_x86.sh
#  "$workspace"/init.linux_armv7.sh
#  "$workspace"/init.macos_x86.sh

#  rustup target add --toolchain ${PLUS_RUST_VERSION} x86_64-unknown-linux-musl
#  rustup target add --toolchain ${PLUS_RUST_VERSION} armv7-unknown-linux-musleabihf
#  rustup target add --toolchain ${PLUS_RUST_VERSION} x86_64-apple-darwin

  rustup component add rustfmt --toolchain ${PLUS_RUST_VERSION}-x86_64-unknown-linux-gnu
  rustup component add clippy --toolchain ${PLUS_RUST_VERSION}-x86_64-unknown-linux-gnu
  rustup override set ${PLUS_RUST_VERSION}
}

setup_other_repositories() {
  git clone https://github.com/tpoechtrager/osxcross.git
}

main

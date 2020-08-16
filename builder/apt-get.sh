#!/usr/bin/env bash

set -xue

main() {
  setup_apt
  echo "done!"
}

setup_apt() {
   apt-get update
   apt-get install -y \
    gcc-arm-linux-gnueabihf \
    jq \
    musl-tools \
    clang \
    cmake \
    tree
}

main

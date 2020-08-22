#!/usr/bin/env bash

set -xue

main() {
  apt-get update
  apt-get install -y \
    gcc-arm-linux-gnueabihf \
    jq \
    musl-tools \
    tree

# todo: add these modules to init.macos_x86.sh
#    clang \
#    cmake \

  echo "done!"
}

main

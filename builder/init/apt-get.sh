#!/usr/bin/env bash

set -xue

main() {
  apt-get update
  apt-get install -y \
    jq \
    xz-utils \
    musl-tools \
    tree

# todo: add these modules to apt-get.macos_x86.sh
#    clang \
#    cmake \

  echo "done!"
}

main

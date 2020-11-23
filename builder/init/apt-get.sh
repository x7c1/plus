#!/usr/bin/env bash

set -xue

main() {
  apt-get update
  apt-get install -y \
    jq \
    xz-utils \
    musl-tools \
    libssl-dev \
    pkg-config \
    tree

  echo "done!"
}

main

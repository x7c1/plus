#!/usr/bin/env bash

set -xue

main() {
  apt-get update
  apt-get install -y \
    jq \
    xz-utils \
    musl-tools \
    tree

  echo "done!"
}

main

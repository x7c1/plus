#!/usr/bin/env bash

set -xue

file=$0
hash=$(sha1sum "$file")
hash_marker="$file".sha1

main() {
  if already_latest; then
    echo "latest modules already installed."
    exit
  fi
  setup_apt
  echo "done!"
  sha1sum "$file" > "$file".sha1
}

already_latest() {
  if [ ! -e "$hash_marker" ]; then
    return 1
  fi
  current=$(cat "$hash_marker")
  if [ "$current" = "$hash" ]; then
    return 0
  else
    return 1
  fi
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

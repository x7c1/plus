#!/usr/bin/env bash

set -xue

file=$0
current_hash=$(sha1sum "$file")
hash_marker="$file".sha1

main() {
  if already_latest; then
    echo "latest modules already installed."
    exit
  fi
  setup_apt
  echo "done!"
  echo "$current_hash" > "$file".sha1
}

already_latest() {
  if [ ! -e "$hash_marker" ]; then
    return 1
  fi
  cached=$(cat "$hash_marker")
  if [ "$cached" = "$current_hash" ]; then
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

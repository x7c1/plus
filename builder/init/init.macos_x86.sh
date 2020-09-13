#!/usr/bin/env bash

set -xue

main () {
  rustup target add --toolchain "${PLUS_RUST_VERSION}" x86_64-apple-darwin

  if is_osxcross_installed; then
    printf "[skip] already installed: osxcross\n\n"
  else
    setup_osxcross
  fi
}

setup_osxcross() {
  cd "$PLUS_PROJECT_ROOT"/osxcross ||  exit 1
  UNATTENDED=1 ./build.sh
}

is_osxcross_installed() {
  target=${OSXCROSS_ROOT}/target/bin/${OSX_SDK_CC}
  if [ -f "${target}" ]; then
    return 0
  else
    return 1
  fi
}

main

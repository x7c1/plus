#!/usr/bin/env bash

set -xue

rustup target add --toolchain "${PLUS_RUST_VERSION}" x86_64-apple-darwin

#main () {
#  rustup target add --toolchain "${PLUS_RUST_VERSION}" x86_64-apple-darwin

#  if is_osxcross_installed; then
#    printf "[skip] already installed: osxcross\n\n"
#  else
#    extract_osxcross
#  fi
#}

#extract_osxcross() {
#  cd "${PLUS_PROJECT_ROOT}/builder"
#  tar -xvf osxcross.tar.xz
#}
#
#setup_osxcross() {
#  if [ ! -d "$OSXCROSS_ROOT" ]; then
#    printf "[skip] not found: osxcross\n\n"
#    return
#  fi
#  cd "$OSXCROSS_ROOT" ||  exit 1
#  UNATTENDED=1 ./build.sh
#}
#
#is_osxcross_installed() {
#  target=${OSXCROSS_ROOT}/target/bin/${OSX_SDK_CC}
#  if [ -f "${target}" ]; then
#    return 0
#  else
#    return 1
#  fi
#}

#main

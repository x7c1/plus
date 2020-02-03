#!/usr/bin/env bash

. ./builder/setup-env.sh
cd "$PROJECT_ROOT" || exit 1

# show executed commands.
set -x

build_for_x86_linux() {
  cargo build \
    --verbose \
    ${BUILD_MODE} \
    --target="${TARGET_X86}"
}

build_for_armv7_linux() {
  CC=arm-linux-gnueabihf-gcc \
  PKG_CONFIG_ALLOW_CROSS=1 \
  cargo build \
    --verbose \
    ${BUILD_MODE} \
    --target="${TARGET_ARMV7}"
}

build_for_macos() {
  if is_osx_sdk_installed; then
    CC=${OSX_SDK_CC} \
    cargo build \
      --verbose \
      ${BUILD_MODE} \
      --target="${TARGET_MACOS}"
  fi
}

build_for_x86_linux

build_for_armv7_linux

build_for_macos
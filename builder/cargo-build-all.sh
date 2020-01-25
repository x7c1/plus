#!/usr/bin/env bash

. ./builder/setup-env.sh
cd "$PROJECT_ROOT" || exit 1

# show executed commands.
set -x

build_for_x86_linux() {
  cargo build \
    --verbose \
    --manifest-path=./apps/s3api/Cargo.toml \
    --features=sabi-s3/x86_support \
    ${BUILD_MODE} \
    --target="${TARGET_X86}"
}

build_for_arm_linux() {
  CC=arm-linux-gnueabihf-gcc \
  PKG_CONFIG_ALLOW_CROSS=1 \
  OPENSSL_STATIC=yes \
  OPENSSL_LIB_DIR=${WSB_WORKSPACE_ARM}/local/lib \
  OPENSSL_INCLUDE_DIR=${WSB_WORKSPACE_ARM}/local/include \
  cargo build \
    --verbose \
    ${BUILD_MODE} \
    --target="${TARGET_ARM}"
}

build_for_mac() {
  if is_osx_sdk_installed; then
    CC=${OSX_SDK_CC} \
    cargo build \
      --verbose \
      --manifest-path=./apps/s3api/Cargo.toml \
      --features=sabi-s3/mac_support \
      ${BUILD_MODE} \
      --target="${TARGET_MAC}"
  fi
}

build_for_x86_linux

build_for_arm_linux

build_for_mac

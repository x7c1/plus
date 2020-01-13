#!/usr/bin/env bash

. ./builder/setup-env.sh
cd "$PROJECT_ROOT" || exit 1

cargo build \
  --verbose \
  ${BUILD_MODE} \
  --target="${TARGET_X86}"

CC=arm-linux-gnueabihf-gcc \
cargo build \
  --verbose \
  ${BUILD_MODE} \
  --target="${TARGET_ARM}"

if is_osx_sdk_installed; then
  CC=${OSX_SDK_CC} \
  cargo build \
    --verbose \
    ${BUILD_MODE} \
    --target="${TARGET_MAC}"
fi

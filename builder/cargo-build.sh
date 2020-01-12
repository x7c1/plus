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

if has_osx_sdk; then
  PATH=${OSXCROSS_ROOT}/target/bin:$PATH \
  CC=${OSX_SDK_CC} \
  cargo build \
    --verbose \
    ${BUILD_MODE} \
    --target="${TARGET_MAC}"
fi

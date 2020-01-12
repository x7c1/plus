#!/usr/bin/env bash

. ./builder/setup-env.sh
cd "$PROJECT_ROOT" || exit 1

# ./builder/cargo-test.sh --debug --opt-level=0

#cargo build \
#  --verbose \
#  ${BUILD_MODE} \
#  --target="${TARGET_X86}"
#
#CC=arm-linux-gnueabihf-gcc \
#cargo build \
#  --verbose \
#  ${BUILD_MODE} \
#  --target="${TARGET_ARM}"

PATH=${OSXCROSS_ROOT}/target/bin:$PATH \
CC=x86_64-apple-darwin19-clang \
cargo build \
  --verbose \
  ${BUILD_MODE} \
  --target="${TARGET_MAC}"

#!/usr/bin/env bash

. ./builder/setup-env.sh
cd "$PROJECT_ROOT" || exit 1

build_for_x86_linux() {
  cargo build \
    --verbose \
    ${BUILD_MODE} \
    --target="${TARGET_X86}"
}

build_for_armv7_linux() {
  CC=arm-linux-gnueabihf-gcc \
  cargo build \
    --verbose \
    ${BUILD_MODE} \
    --target="${TARGET_ARMV7}"
}

build_for_mac() {
  if is_osx_sdk_installed; then
    CC=${OSX_SDK_CC} \
    cargo build \
      --verbose \
      ${BUILD_MODE} \
      --target="${TARGET_MACOS}"
  fi
}

s3api() {
  "$PROJECT_ROOT"/target/x86_64-unknown-linux-musl/debug/s3api $@
}

run_put() {
  s3api put-object \
    --bucket=wasabi-2020b-dev \
    --key=README.md \
    --body=README.md
}

run_single_test() {
  cargo test \
    ${BUILD_MODE} \
    --target="${TARGET_X86}" \
    --package=sabi-core \
    -- --nocapture auth::account
}

run_all_tests() {
  cargo test \
    ${BUILD_MODE} \
    --target="${TARGET_X86}" \
    -- --nocapture
}

set -x

#build_for_x86_linux
#run_put

run_single_test

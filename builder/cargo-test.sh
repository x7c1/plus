#!/usr/bin/env bash

. ./builder/setup-env.sh
cd "$PROJECT_ROOT" || exit 1

cargo build \
  --verbose \
  ${BUILD_MODE} \
  --target="${TARGET_X86}"
#
#cargo build \
#  --verbose \
#  ${BUILD_MODE} \
#  --target="${TARGET_ARM}"

cargo test \
  ${BUILD_MODE} \
  --target="${TARGET_X86}" \
  -- --nocapture

#cargo run \
#  ${BUILD_MODE} \
#  --target="${TARGET_X86}"

"$PROJECT_ROOT"/target/x86_64-unknown-linux-musl/debug/s3api \
  put-object \
    --bucket=hoge \
    --key=fuga

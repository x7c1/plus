#!/usr/bin/env bash

. ./builder/setup-env.sh
cd "$PROJECT_ROOT" || exit 1

cargo test \
  ${BUILD_MODE} \
  --target="${TARGET_X86}" \
  -- --nocapture

"$PROJECT_ROOT"/target/x86_64-unknown-linux-musl/debug/s3api \
  put-object \
    --bucket=hoge \
    --key=fuga

"$PROJECT_ROOT"/target/x86_64-unknown-linux-musl/debug/s3api \
  get-object \
    --bucket=hoge \
    --key=fuga \
    piyo

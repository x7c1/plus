#!/usr/bin/env bash

. ./builder/setup-env.sh
cd "$PROJECT_ROOT" || exit 1

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

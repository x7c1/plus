#!/usr/bin/env bash

# Usage:
# 1.
#   $ cp ./builder/draft.sample.sh ./builder/draft.local.sh
# 2.
#   $ ./scripts/run_draft.sh

. ./builder/setup-env.sh

cd "$PROJECT_ROOT" || exit 1

. ./builder/lib.linux_x86.sh

main() {
  set -x

  build_apps_for_linux_x86

  run_unit_tests_for_linux_x86

  cargo_fmt

  cargo_clippy

  run_specified_test
}

s3api() {
  "$PROJECT_ROOT"/target/x86_64-unknown-linux-musl/debug/s3api $@
}

run_put() {
  s3api put-object \
    --bucket=my-test-bucket \
    --key=README.md \
    --body=README.md
}

run_get() {
  s3api get-object \
    --bucket=my-test-bucket \
    --key=README.md \
    test.README.md
}

run_specified_test() {
  cargo test \
    ${BUILD_MODE} \
    --target="${TARGET_X86}" \
    --workspace --exclude=wsb-pilot \
    -- --nocapture auth::v4::calculator
}

main

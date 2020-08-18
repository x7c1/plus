#!/usr/bin/env bash

# Usage:
# 1.
#   $ cp ./builder/draft.sample.sh ./builder/draft.local.sh
# 2.
#   $ ./scripts/run_draft.sh

. ./builder/setup-env.sh

cd "$PLUS_PROJECT_ROOT" || exit 1
. ./builder/lib.linux_x86.sh

main() {
  set -x
  for_task_runner
}

for_task_runner() {
  task_runner_for_linux_x86 build-apps --target=linux_x86
  task_runner_for_linux_x86 assemble-pilot-tests --target=linux_x86
  task_runner_for_linux_x86 copy-artifact-files --target=linux_x86
  task_runner_for_linux_x86 package-artifacts --target=linux_x86
  ls -lh dist

  task_runner_for_linux_x86 strip-executables --target=linux_x86
  task_runner_for_linux_x86 package-artifacts --target=linux_x86
  ls -lh dist

  task_runner_for_linux_x86 help
}

dev() {
  build_apps_for_linux_x86
  run_unit_tests_for_linux_x86
  cargo_fmt
  cargo_clippy
  run_specified_tests
}

s3api() {
  "$PLUS_PROJECT_ROOT"/target/x86_64-unknown-linux-musl/debug/s3api $@
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

run_specified_tests() {
  cargo test \
    ${BUILD_MODE} \
    --target="${TARGET_X86}" \
    --workspace --exclude=plus-pilot \
    -- --nocapture auth::v4::calculator
}

run_package_specified_tests() {
  cargo test \
    ${BUILD_MODE} \
    --target="${TARGET_X86}" \
    --package wsb-task \
    -- --nocapture core
}

main

#!/usr/bin/env bash

# Usage:
# $ make draft

. ./builder/assemble.sh

cd "$PLUS_PROJECT_ROOT" || exit 1
. ./builder/lib.linux_x86.sh

main() {
  set -x
  task release-libraries '--files=[".github/workflows/assemble.yml","Makefile","apps/plus-task/src/tasks/mod.rs","apps/plus-task/src/tasks/release_libraries/mod.rs","apps/plus-task/src/tasks/release_libraries/task.rs","builder/assemble.sh","libs/env-extractor/Cargo.toml","scripts/run_builder.sh"]'
  for_task_runner
}

for_task_runner() {
  task build-apps --target=linux_x86
  task assemble-pilot-tests --target=linux_x86
  task copy-artifact-files --target=linux_x86
  task package-artifacts --target=linux_x86
  ls -lh dist

  task strip-executables --target=linux_x86
  task package-artifacts --target=linux_x86
  ls -lh dist

  task help
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
  export RUSTFLAGS="-C opt-level=0"
  cargo test \
    --target="x86_64-unknown-linux-musl" \
    --workspace --exclude=plus-pilot \
    -- --nocapture auth::v4::calculator
}

run_package_specified_tests() {
  export RUSTFLAGS="-C opt-level=0"
  cargo test \
    --target="x86_64-unknown-linux-musl" \
    --package plus-task \
    -- --nocapture core
}

main

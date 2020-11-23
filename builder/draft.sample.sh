#!/usr/bin/env bash

# Usage:
# $ make draft

. ./builder/assemble.sh

cd "$PLUS_PROJECT_ROOT" || exit 1

main() {
  set -x
  release_apps
}

s3api() {
  "$PLUS_PROJECT_ROOT"/target/x86_64-unknown-linux-musl/debug/s3api $@
}

release_apps() {
  export GITHUB_TOKEN="xxxxx"

  # rf. https://github.community/t/github-actions-bot-email-address/17204/4
  task release-apps \
    --files='["apps/s3api/Cargo.toml"]'
}

package_artifacts() {
  task package-artifacts \
    --release \
    --target=x86_64-unknown-linux-musl \
    --files='["apps/s3api/Cargo.toml"]'
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

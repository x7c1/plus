#!/usr/bin/env bash

cargo_fmt() {
  cargo fmt --verbose -- --emit files
}

build_apps() {
  cargo build \
    --verbose \
    ${BUILD_MODE} \
    --target="${TARGET_X86}"
}

run_unit_tests() {
  cargo test \
    ${BUILD_MODE} \
    --target="${TARGET_X86}" \
    --workspace --exclude=wsb-pilot \
    -- --nocapture
}

build_e2e_tests() {
  cargo test \
    ${BUILD_MODE} \
    --target="${TARGET_X86}" \
    --package=wsb-pilot \
    --no-run \
    --message-format=json \
    | jq -r "select(.profile.test == true) | .filenames[]" \
    > ${PROJECT_ROOT}/draft.tmp

    cat ${PROJECT_ROOT}/draft.tmp
}

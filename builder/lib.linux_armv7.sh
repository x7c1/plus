#!/usr/bin/env bash

build_apps() {
  cargo build \
    --verbose \
    ${BUILD_MODE} \
    --target="${TARGET_ARMV7}"
}

build_e2e_tests() {
  cargo test \
    ${BUILD_MODE} \
    --target="${TARGET_ARMV7}" \
    --package=wsb-pilot \
    --no-run \
    --message-format=json \
    | jq -r "select(.profile.test == true) | .filenames[]" \
    > ${PROJECT_ROOT}/draft.tmp

    cat ${PROJECT_ROOT}/draft.tmp
}

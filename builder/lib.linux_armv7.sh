#!/usr/bin/env bash

build_apps_for_linux_armv7() {
  cargo build \
    --verbose \
    ${BUILD_MODE} \
    --target="${TARGET_ARMV7}"
}

build_e2e_tests_for_linux_armv7() {
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

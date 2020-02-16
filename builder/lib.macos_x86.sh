#!/usr/bin/env bash

build_apps() {
  if ! is_osx_sdk_installed; then
    return
  fi
  cargo build \
    --verbose \
    ${BUILD_MODE} \
    --target="${TARGET_MACOS}"
}

build_e2e_tests() {
  if ! is_osx_sdk_installed; then
    return
  fi
  cargo test \
    ${BUILD_MODE} \
    --target="${TARGET_MACOS}" \
    --package=wsb-pilot \
    --no-run \
    --message-format=json \
    | jq -r "select(.profile.test == true) | .filenames[]" \
    > ${PROJECT_ROOT}/draft.tmp

    cat ${PROJECT_ROOT}/draft.tmp
}

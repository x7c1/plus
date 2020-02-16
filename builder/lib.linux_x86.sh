#!/usr/bin/env bash

build_apps_for_linux_x86() {
  build_apps ${TARGET_X86}
}

build_e2e_tests_for_linux_x86() {
  build_e2e_tests ${TARGET_X86}
}

run_unit_tests_for_linux_x86() {
  cargo test \
    ${BUILD_MODE} \
    --target="${TARGET_X86}" \
    --workspace --exclude=wsb-pilot \
    -- --nocapture
}

cargo_fmt() {
  cargo fmt --verbose -- --emit files
}

#!/usr/bin/env bash

build_apps_for_linux_x86() {
  build_apps ${TARGET_X86}
}

build_e2e_tests_for_linux_x86() {
  build_e2e_tests ${TARGET_X86}
}

copy_release_apps_for_linux_x86() {
  copy_release_apps ${TARGET_X86} "linux_x86"
}

strip_release_files_for_linux_x86() {
  strip_release_files "linux_x86" strip
}

show_artifacts_for_linux_x86() {
  show_artifacts "linux_x86"
}

run_unit_tests_for_linux_x86() {
  cargo test \
    ${BUILD_MODE} \
    --target="${TARGET_X86}" \
    --workspace --exclude=plus-pilot \
    -- --nocapture
}

cargo_fmt() {
  cargo fmt \
    --verbose \
    -- --emit files
}

#cargo_clippy() {
#  cargo clippy \
#    ${BUILD_MODE} \
#    --target="${TARGET_X86}"
#}

task_runner_for_linux_x86() {
  task_runner ${TARGET_X86} "$@"
}

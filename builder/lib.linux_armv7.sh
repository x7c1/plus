#!/usr/bin/env bash

build_apps_for_linux_armv7() {
  CC=${ARMV7_CC} \
  build_apps ${TARGET_ARMV7}
}

build_e2e_tests_for_linux_armv7() {
  build_e2e_tests ${TARGET_ARMV7}
}

copy_release_apps_for_linux_armv7() {
  if ! is_osx_sdk_installed; then
    return
  fi
  copy_release_apps ${TARGET_ARMV7} "linux_armv7"
}

strip_release_files_for_linux_armv7() {
  strip_release_files "linux_armv7" arm-linux-gnueabihf-strip
}

show_artifacts_for_linux_armv7() {
  show_artifacts "linux_armv7"
}

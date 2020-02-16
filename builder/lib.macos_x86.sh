#!/usr/bin/env bash

build_apps_for_macos_x86() {
  if ! is_osx_sdk_installed; then
    return
  fi
  CC=${OSX_SDK_CC} \
  build_apps ${TARGET_MACOS}
}

build_e2e_tests_for_macos_x86() {
  if ! is_osx_sdk_installed; then
    return
  fi
  build_e2e_tests ${TARGET_MACOS}
}

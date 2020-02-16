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

copy_release_apps_for_macos_x86() {
  if ! is_osx_sdk_installed; then
    return
  fi
  copy_release_apps ${TARGET_MACOS} "macos_x86"
}

strip_release_files_for_macos_x86() {
  if ! is_osx_sdk_installed; then
    return
  fi
  strip_release_files "macos_x86" x86_64-apple-darwin19-strip
}

show_artifacts_for_macos_x86() {
  show_artifacts "macos_x86"
}

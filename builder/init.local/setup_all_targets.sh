#!/usr/bin/env bash

set -xue

export DEBIAN_FRONTEND="noninteractive"

main() {
  setup_shared_modules
  setup_linux_x86
  setup_linux_armv7
  setup_macos_x86
}

run() {
  ./builder/init/run-if-changed.sh "$1"
}

setup_shared_modules() {
  run ./builder/init/apt-get.sh
  . ./builder/init.local/setup_rust.sh
  . ./builder/init/env.sh
  run ./builder/init/init.sh
}

setup_linux_x86() {
  run ./builder/init/init.linux_x86.sh
  run ./builder/init/tools.linux_x86.sh
  run ./builder/init/release.linux_x86.sh
}

setup_linux_armv7() {
  run ./builder/init/apt-get.linux_armv7.sh
  run ./builder/init/init.linux_armv7.sh
}

setup_macos_x86() {
  . ./builder/init/env.macos_x86.sh

  if ! has_osx_sdk; then
    printf "[skip] not found: %s\n\n" "$OSX_SDK_TARBALL"
    return
  fi

  run ./builder/init/apt-get.macos_x86.sh
  run ./builder/init/init.macos_x86.sh
}

has_osx_sdk() {
  result=$(find ./builder -name "$OSX_SDK_TARBALL" -print)
  if [ -z "$result" ]; then
    return 1
  else
    return 0
  fi
}

main

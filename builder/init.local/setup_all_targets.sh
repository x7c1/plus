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
}

setup_linux_armv7() {
  run ./builder/init/apt-get.linux_armv7.sh
  run ./builder/init/init.linux_armv7.sh
}

setup_macos_x86() {
  run ./builder/init/apt-get.macos_x86.sh
  . ./builder/init/env.macos_x86.sh
  run ./builder/init/init.macos_x86.sh
}

main

#!/usr/bin/env bash

set -xue

export DEBIAN_FRONTEND="noninteractive"

main() {
  run ./builder/init/apt-get.sh
  run ./builder/init/apt-get.linux_armv7.sh

  . ./builder/init.local/setup_rust.sh
  . ./builder/init/env.sh
  . ./builder/init/env.macos_x86.sh

  run ./builder/init/init.sh
  run ./builder/init/init.linux_x86.sh
  run ./builder/init/tools.linux_x86.sh

  run ./builder/init/init.linux_armv7.sh
  run ./builder/init/init.macos_x86.sh
}

run() {
  ./builder/init/run-if-changed.sh "$1"
}

main

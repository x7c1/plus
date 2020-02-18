#!/usr/bin/env bash

. ./builder/setup-env.sh
. ./builder/lib.linux_x86.sh
cd "$PROJECT_ROOT" || exit 1

main() {
  # show executed commands.
  set -x

  cargo_fmt

  build_apps_for_linux_x86

  run_unit_tests_for_linux_x86
}

main $@

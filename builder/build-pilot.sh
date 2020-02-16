#!/usr/bin/env bash

. ./builder/setup-env.sh
cd "$PROJECT_ROOT" || exit 1

. ./builder/lib.linux_x86.sh

set -x

cargo_fmt

build_apps_for_linux_x86

run_unit_tests_for_linux_x86

build_e2e_tests_for_linux_x86 \
  | sed s/^\\${PROJECT_ROOT}/\\./ \
  > ${PROJECT_ROOT}/scripts/run_pilot_path.tmp

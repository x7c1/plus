#!/usr/bin/env bash

. ./builder/setup-env.sh
cd "$PROJECT_ROOT" || exit 1

# show executed commands.
set -x

for arch in $(get_arch_labels); do
  . ./builder/lib.${arch}.sh
  build_apps_for_${arch}

  file_path=$(build_e2e_tests_for_${arch})
done

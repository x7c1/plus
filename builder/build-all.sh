#!/usr/bin/env bash

. ./builder/setup-env.sh
cd "$PROJECT_ROOT" || exit 1

# show executed commands.
set -x

for arch in $(get_arch_labels); do
  arch_dir="${ARTIFACTS_DIR}/${arch}"

  . ./builder/lib.${arch}.sh
  build_apps_for_${arch}

  file_path=$(build_e2e_tests_for_${arch})
  if [[ -n "$file_path" ]]; then
    cp "$file_path" "$arch_dir/wsb_pilot_tests"
  fi
  # create runner script.
  template="./dist.bundle/run_pilot_tests.sh.template"
  cp $template "$arch_dir/run_pilot_tests.sh"
done

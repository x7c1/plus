#!/usr/bin/env bash

. ./builder/setup-env.sh
cd "$PROJECT_ROOT" || exit 1

# show executed commands.
set -x

for arch in $(get_arch_labels); do
  arch_dir="${ARTIFACTS_DIR}/${arch}"

  . ./builder/lib.${arch}.sh
  copy_release_apps_for_${arch}
done

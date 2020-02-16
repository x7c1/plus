#!/usr/bin/env bash

. ./builder/setup-env.sh
cd "$PROJECT_ROOT" || exit 1

for arch in $(get_arch_labels); do
  . ${PROJECT_ROOT}/builder/lib.${arch}.sh
  show_artifacts_for_${arch}
done

#!/usr/bin/env bash

. ./builder/setup-env.sh
cd "$PLUS_PROJECT_ROOT" || exit 1

for arch in $(get_arch_labels); do
  . ./builder/lib.${arch}.sh
  strip_release_files_for_${arch}
done

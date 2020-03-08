#!/usr/bin/env bash

. ./builder/setup-env.sh
cd "$PROJECT_ROOT" || exit 1

# show executed commands.
set -x

for arch in $(get_arch_labels); do
  arch_dir="${ARTIFACTS_DIR}/${arch}"

  # create archive.
  tar -Jcf "$arch_dir.tar.xz" -C "${ARTIFACTS_DIR}" "$arch"
done

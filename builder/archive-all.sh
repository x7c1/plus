#!/usr/bin/env bash

. ./builder/setup-env.sh
cd "$PROJECT_ROOT" || exit 1

# show executed commands.
set -x

export XZ_OPT="-9e --threads=0"

for arch in $(get_arch_labels); do
  arch_dir="${ARTIFACTS_DIR}/${arch}"

  # create archive.
  tar --xz \
    --create --file "$arch_dir.tar.xz" \
    --directory "${ARTIFACTS_DIR}" \
    "$arch" &
done

wait

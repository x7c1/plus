#!/usr/bin/env bash

MOUNT_DIR=/plus
IMAGE_NAME=ghcr.io/x7c1/osxcross:0.4
CONTAINER_NAME=osxcross-loader

sdk="MacOSX10.15.sdk.tar.xz"
if [ -f ./builder/"$sdk" ]; then
  echo "[skip] already downloaded: $sdk"
  exit
fi

if [ -f "./builder/osxcross/tarballs/$sdk" ]; then
  echo "[skip] already downloaded: $sdk"
  exit
fi

set -x
docker run \
    --privileged \
    --volume "$(pwd)":"${MOUNT_DIR}" \
    --name "${CONTAINER_NAME}" \
    --tty \
    --workdir "${MOUNT_DIR}" \
    --rm \
    --entrypoint="" \
    "${IMAGE_NAME}" \
    cp /workspace/"$sdk" "$MOUNT_DIR/builder/"

echo "[done] downloaded: ./builder/$sdk"

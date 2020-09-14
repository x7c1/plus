#!/usr/bin/env bash

MOUNT_DIR=/plus
IMAGE_NAME=ghcr.io/x7c1/osxcross:0.3
CONTAINER_NAME=osxcross-loader

sdk="MacOSX10.15.sdk.tar.bz2"
if [ -f "$MOUNT_DIR/$sdk" ]; then
  echo "[skip] already downloaded: $sdk"
  exit
fi

if [ -f "$MOUNT_DIR/builder/osxcross/tarballs/$sdk" ]; then
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
    cp /workspace/"$sdk" "$MOUNT_DIR/"

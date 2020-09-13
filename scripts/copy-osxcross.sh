#!/usr/bin/env bash

MOUNT_DIR=/plus
IMAGE_NAME=ghcr.io/x7c1/osxcross
CONTAINER_NAME=osxcross-loader

docker run \
    --privileged \
    --volume "$(pwd)":"${MOUNT_DIR}" \
    --name "${CONTAINER_NAME}" \
    --tty \
    --workdir "${MOUNT_DIR}" \
    --rm \
    --entrypoint="" \
    "${IMAGE_NAME}" \
    cp -r /workspace/osxcross "${MOUNT_DIR}"/builder

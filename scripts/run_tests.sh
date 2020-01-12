#!/usr/bin/env bash

# stop if non-zero returned.
set -e

# not allow undefined values.
set -u

# show executed commands.
set -x

CONTAINER_NAME=wasabi_builder_cacher
IMAGE_NAME=wasabi_builder
MOUNT_DIR=/wasabi
ARGS=$@
COMMAND="sh ${MOUNT_DIR}/builder/cargo-test.sh ${ARGS} --debug --opt-level=0"

. ./scripts/run_cacher.sh

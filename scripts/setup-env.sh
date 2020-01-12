#!/usr/bin/env bash

# stop if non-zero returned.
set -e

# not allow undefined values.
set -u

# show executed commands.
set -x

IMAGE_NAME=wasabi_builder
MOUNT_DIR=/wasabi

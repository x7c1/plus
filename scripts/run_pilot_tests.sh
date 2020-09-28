#!/usr/bin/env bash

# Usage:
# 1.
#   $ cp ./scripts/setup_pilot_env.local.template ./scripts/setup_pilot_env.local.sh
# 2.
#   $ vim ./scripts/setup_pilot_env.local.sh
# 3.
#   $ ./scripts/run_pilot_tests.sh

# See also:
# ./scripts/setup_aws_env.sh

# u: not allow undefined values.
# e: stop if non-zero returned.
set -ue

./scripts/run_builder.sh assemble_debug_pilot
. ./scripts/setup_pilot_env.local.sh

MOUNT_DIR="/plus"
IMAGE_NAME="amazon/aws-cli:2.0.50"
CONTAINER_NAME="pilot-tests-runner"

docker run \
    --privileged \
    --volume "$(pwd)":"${MOUNT_DIR}" \
    --name "${CONTAINER_NAME}" \
    --tty \
    --workdir "$MOUNT_DIR" \
    --env PLUS_WORKSPACE_DIR="$MOUNT_DIR/dist/linux_x86-debug/plus_pilot_workspace" \
    --env PLUS_APPS_DIR="$MOUNT_DIR/dist/linux_x86-debug" \
    --env PLUS_TEST_BUCKET="$PLUS_TEST_BUCKET" \
    --env AWS_ACCESS_KEY_ID="$AWS_ACCESS_KEY_ID" \
    --env AWS_SECRET_ACCESS_KEY="$AWS_SECRET_ACCESS_KEY" \
    --env AWS_DEFAULT_REGION="$AWS_DEFAULT_REGION" \
    --rm \
    --entrypoint="" \
    "$IMAGE_NAME" \
    "$MOUNT_DIR"/dist/linux_x86-debug/plus_pilot_tests --nocapture --color always

# Print stdout:
# ./dist/linux_x86-debug/plus_pilot_tests --nocapture

# Disable parallel run:
# ./dist/linux_x86-debug/plus_pilot_tests --nocapture --test-threads=1

# Colorize:
# ./dist/linux_x86-debug/plus_pilot_tests --nocapture --color always

# Filter target tests
# ./dist/linux_x86-debug/plus_pilot_tests get_object --nocapture --color always

# See also:
# https://doc.rust-lang.org/book/ch11-02-running-tests.html

#!/usr/bin/env bash

. ./scripts/setup-env.sh

CONTAINER_NAME=wasabi_builder_cacher
ARGS=$@
COMMAND="sh ${MOUNT_DIR}/builder/cargo-test.sh ${ARGS} --debug --opt-level=0"

. ./scripts/run_cacher.sh

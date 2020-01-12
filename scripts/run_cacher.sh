#!/usr/bin/env bash

. ./scripts/setup-env.sh

IMAGE_NAME=wasabi_builder
MOUNT_DIR=/wasabi
CONTAINER_NAME=wasabi_builder_cacher

write_main "${MOUNT_DIR}/builder/$(echo $@)"

if [[ "$(current_container)" ]]; then
  restart_container
else
  run_container
fi

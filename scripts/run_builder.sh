#!/usr/bin/env bash

. ./scripts/setup_env.sh

MOUNT_DIR=/plus

write_main "${MOUNT_DIR}/builder/$(echo $@)"

if [[ "$(current_container)" ]]; then
  restart_container
else
  run_container
fi

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

current_container() {
  docker inspect --format '{{.Created}}' ${CONTAINER_NAME}
}

run_container() {
  docker run \
      --privileged \
      --volume $(pwd):${MOUNT_DIR} \
      --name ${CONTAINER_NAME} \
      --tty \
      --workdir ${MOUNT_DIR} \
      ${IMAGE_NAME} \
      sh ${MOUNT_DIR}/builder/cargo-test.sh ${ARGS} --debug --opt-level=0
}
#      --user ${UID}:$(id -g) \


restart_container() {
  docker start --attach ${CONTAINER_NAME}
}

if [[ "$(current_container)" ]]; then
  restart_container
else
  run_container
fi

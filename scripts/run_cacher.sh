#!/usr/bin/env bash

current_container() {
  docker inspect --format '{{.Created}}' ${CONTAINER_NAME}
}

run_container() {
  docker run \
      --privileged \
      --volume "$(pwd)":${MOUNT_DIR} \
      --name ${CONTAINER_NAME} \
      --tty \
      --workdir ${MOUNT_DIR} \
      ${IMAGE_NAME} \
      ${COMMAND}
}

restart_container() {
  docker start --attach ${CONTAINER_NAME}
}

if [[ "$(current_container)" ]]; then
  restart_container
else
  run_container
fi

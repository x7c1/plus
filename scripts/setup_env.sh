#!/usr/bin/env bash

# stop if non-zero returned.
set -e

# not allow undefined values.
set -u

current_container() {
  docker inspect --format '{{.Created}}' "${CONTAINER_NAME}"
}

run_container() {
  docker run \
      --privileged \
      --volume "$(pwd)":"${MOUNT_DIR}" \
      --name "${CONTAINER_NAME}" \
      --tty \
      --workdir "${MOUNT_DIR}" \
      --env PLUS_PROJECT_ROOT="${MOUNT_DIR}" \
      "${IMAGE_NAME}" \
      sh "${MOUNT_DIR}"/builder/main.gen.sh
}

restart_container() {
  docker start --attach ${CONTAINER_NAME}
}

write_main() {
  path="./builder/main.gen.sh"
  echo -e "#!/usr/bin/env bash\n" > ${path}
  echo -e ". ${MOUNT_DIR}/builder/init.local/setup_all_targets.sh\n" >> ${path}
  echo -e ". ${MOUNT_DIR}/builder/assemble.sh\n" >> ${path}
  echo -e "$1" >> ${path}
}

export IMAGE_NAME="ubuntu:latest"
export CONTAINER_NAME="plus_builder"

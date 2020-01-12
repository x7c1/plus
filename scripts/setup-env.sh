#!/usr/bin/env bash

# stop if non-zero returned.
set -e

# not allow undefined values.
set -u

# show executed commands.
set -x

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
      sh ${MOUNT_DIR}/builder/main.gen.sh
}

restart_container() {
  docker start --attach ${CONTAINER_NAME}
}

write_main() {
  text=$(cat << EOS
#!/usr/bin/env bash

$1
EOS
)
  echo "$text" > ./builder/main.gen.sh
}

export CONTAINER_NAME=wasabi_builder_cacher

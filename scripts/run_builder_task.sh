#!/bin/bash

# stop if non-zero returned.
set -e

CONTAINER_NAME=wasabi_builder_task_runner
IMAGE_NAME=wasabi_builder
MOUNT_DIR=/wasabi

TARGET=$1
SCRIPTS_DIR="builder"

if [[ -z ${TARGET} ]]; then
    labels=$(ls ./${SCRIPTS_DIR} | grep '.sh$' | sed 's/.sh$//')
    echo "Usage: ./scripts/run_builder_task.sh <name>"
    echo "<name>:"
    for label in ${labels}
    do
        echo "  $label"
    done
    exit 1
fi

# not allow undefined values.
set -u

# show executed commands.
set -x

# exclude $1
ARGS=${@:2}

docker run \
    --privileged \
    --volume "$(pwd)":${MOUNT_DIR} \
    --name ${CONTAINER_NAME} \
    --interactive \
    --rm \
    --tty \
    --workdir ${MOUNT_DIR} \
    ${IMAGE_NAME} \
    sh ${MOUNT_DIR}/${SCRIPTS_DIR}/"${TARGET}".sh $ARGS

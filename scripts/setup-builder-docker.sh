#!/usr/bin/env bash

set -x

IMAGE_NAME=wasabi_builder

docker build ./builder \
    --tag ${IMAGE_NAME}

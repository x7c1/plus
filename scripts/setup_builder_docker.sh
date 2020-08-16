#!/usr/bin/env bash

. ./scripts/setup_env.sh

set -x

docker build ./builder --tag "${IMAGE_NAME}"

#!/usr/bin/env bash

. ./scripts/setup_env.sh

repository="x7c1/wasabi"

docker commit ${CONTAINER_NAME} ${repository}

docker push ${repository}:latest
